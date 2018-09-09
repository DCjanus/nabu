# 订阅源开发指引

## 预备知识

+ 项目中存在两个重要概念：`Source`与`FeedGenerator`
+ 每个`Source`对应一个网站，如GitHub、V2EX
+ 每个`FeedGenerator`对应一个类型的`RSS Feed`，如某个GitHub用户的Repository列表、V2EX每日热帖列表
+ 每个`Source`对应一个到多个`FeedGenerator`

## 目录结构

```text                      
src
├── atom_hub.rs
├── config.rs
├── database
│   ├── init.sql
│   └── mod.rs
├── feed_generator.rs               # Trait FeedGenerator定义
├── main.rs
├── routes                          # 所有实现订阅源都在该文件夹下
│   ├── github                      
│   │   ├── mod.rs                  # 示例：Github Source
│   │   └── user_repos.rs           # 示例：某GitHub用户repository列表的RSS订阅
│   ├── mod.rs
│   └── v2ex
│       ├── mod.rs                  # 示例：V2EX Source
│       └── hot_topics.rs           # 示例：V2EX每日热帖的RSS订阅
├── source.rs
└── utils.rs
```

## 详细指引

### 实现FeedGenerator

项目中存在一个名为`FeedGenerator`的Trait，一般RSS源开发者**需要关心**的`FeedGenerator`定义如下:

```rust
pub trait FeedGenerator {                               // 对应一个RSS Feed
    type Info: DeserializeOwned + Serialize + Default;  // 用于规定对Query String的解析方式
    const PATH: &'static str;                           // 用于规定最终该RSS Feed的URL path 
    fn update(info: &Self::Info) -> NabuResult<Feed>;   // 用于接受用户请求、生成对应RSS Feed
}
```

主要的生成逻辑应实现于update方法中，其接受一个根据用户请求URL的Query String生成的结构体，并返回生成的Feed或生成过程中发生的错误。

#### FeedGenerator::Info

在当前版本的设计中，每个`FeedGenerator`对应一个URL Path，且该Path中不能存在任何变量。对于部分RSS源，需要由请求者传入参数，此时可以使用Query String。

当前版本使用[serde_qs](https://crates.io/crates/serde_qs)解析Query String，也就意味着默认情况下，项目将支持在Query String中表达键值对、数组等特性，即其表现力无限接近`JSON`，如

```text
name=Acme&id=42&phone=12345&address[postcode]=12345&\
    address[city]=Carrot+City&user_ids[0]=1&user_ids[1]=2&\
    user_ids[2]=3&user_ids[3]=4
```

等价于

```json
{
  "name": "Acme",
  "id": 42,
  "phone": 12345,
  "address": {
    "postcode": 12345,
    "city": "Carrot City"
  },
  "user_ids": [1, 2, 3, 4]
}
```

开发者可以通过自定义Info实际类型，并在[serde文档](https://serde.rs/)的帮助下，实现对Query String的解析逻辑，如：

```rust
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct UserRepoInfo {
    username: String,
    #[serde(rename = "type")]
    ty: String,
    sort: String,
    direction: String,
}

impl Default for UserRepoInfo {
    fn default() -> Self {
        UserRepoInfo {
            username: "DCjanus".to_string(),
            ty: "owner".to_string(),
            sort: "full_name".to_string(),
            direction: "desc".to_string(),
        }
    }
}
```

随后将其类型指定为`FeedGenerator::Info`的实际类型，如:

```rust
pub struct UserRepoGenerator;
impl FeedGenerator for UserRepoGenerator {
    type Info = UserRepoInfo;
    // ... other code
}
```

#### FeedGenerator::update

update中包含`FeedGenerator`的主要实现逻辑。

需要注意的是，受限于版权因素，不应在生成的Feed中包含完整内容，即避免使用Content字段，并保证Description字段中不包含完整内容。

#### FeedGenerator::PATH

指定该`FeedGenerator`部分URL Path，同一Source下的所有`FeedGenerator`该字段不应相同。示例:

```rust
impl FeedGenerator for UserRepoGenerator {
    const PATH: &'static str = "user/repos";
}
```

### 实现IntoSource

以GitHub为例，每个网站对应的所有RSS源都应注册到同一`Source`上，在当前版本中，`Source`是一个预先提供的Struct，对于订阅源开发者，实际应创建一个名称以'Source'结尾的Struct，如`GitHubSource`，并为其实现名为`IntoSource`的Trait，如:

```rust
impl IntoSource for GitHubSource {
    fn into_source(self) -> Source {
        Source::new("github")               // 该Source的prefix
        .register(UserRepoGenerator)        // 注册相关FeedGenerator 
        .register(UnimplementedGenerator1)
        .register(UnimplementedGenerator2)
    }
}
```

### 注册Source

在`{PROJECT_ROOT}/src/routes/mod.rs`中，定义了名为`atom_hub`的函数，其实现如下:

```rust
pub fn atom_hub() -> AtomHub {
    AtomHub::default()
        .register(::routes::github::GitHubSource)
        .register(::routes::v2ex::V2exSource)
}
```

模仿已有代码，将实现了`IntoSource`的结构体实例注册到`AtomHub`实例中即可。

## 其他

每个`FeedGenerator`对应的URL Path为:`/{Source Prefix}/{FeedGenerator path}`，如`GitHubSource`的prefix为`/github`，`UserRepoGenerator`的path为`/user/repos`，则最终其URL Path为`/github/user/repos`。
