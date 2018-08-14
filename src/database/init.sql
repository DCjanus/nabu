create table if not exists fetch_cache (
  pk           bigserial primary key,
  path         text                                               not null,
  info         jsonb                                              not null,
  updated_time timestamp with time zone default current_timestamp not null,
  content      text                                               not null,
  constraint logic_unique_key unique (path, info)
);

create or replace function updated_time_modifier()
  returns trigger as $updated_time_modifier$
begin
  if row (new.*) is distinct from row (old.*)
  then
    new.updated_time = now();
    return new;
  else
    return old;
  end if;
end;
$updated_time_modifier$
language 'plpgsql';

drop trigger if exists updated_time_modifier
on fetch_cache;

create trigger updated_time_modifier
  before update
  on fetch_cache
  for each row execute procedure updated_time_modifier();

