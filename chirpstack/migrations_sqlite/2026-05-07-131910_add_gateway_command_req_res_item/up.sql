create table gateway_command_req_res_item
(
    id          text     not null primary key,
    gateway_id  blob     not null references gateway on delete cascade,
    created_at  datetime not null,
    exec_id     integer  not null,
    command     text     not null,
    stdin       blob     not null,
    environment text     not null default '{}',
    response_at datetime,
    stdout      blob,
    stderr      blob,
    error       text
);

create index idx_gateway_command_req_res_item_gateway_id_exec_id on gateway_command_req_res_item (gateway_id, exec_id);