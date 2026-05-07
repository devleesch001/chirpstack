create table gateway_command_req_res_item
(
    id          uuid primary key,
    gateway_id  bytea                    not null references gateway on delete cascade,
    created_at  timestamp with time zone not null,
    exec_id     integer                  not null,
    command     text                     not null,
    stdin       bytea                    not null,
    environment jsonb                    not null default '{}'::jsonb,
    response_at timestamp with time zone,
    stdout      bytea,
    stderr      bytea,
    error       text
);

create index idx_gateway_command_req_res_item_gateway_id_exec_id on gateway_command_req_res_item (gateway_id, exec_id);