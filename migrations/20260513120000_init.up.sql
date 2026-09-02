-- Initial schema for skw-auth-service.

-- Scope of a role's reach within the entity tree.
CREATE TYPE role_scope AS ENUM ('AllChilds', 'FullTree', 'SelfOnly');

CREATE TABLE roles (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name            TEXT NOT NULL CHECK (length(trim(name)) > 0),
    role_scope      role_scope NOT NULL,
    role_service    TEXT NOT NULL DEFAULT '*', -- service name entity and action related to etc. 'skw-orders-service' '*' for any service;
    role_entities   TEXT[], -- external service entity name etc. 'orders' or '*' for any entity
    role_actions    TEXT[] -- external service entity actions etc. 'read', or '*' for any action
);

-- Entities form a tree; the root is self-parented.
CREATE TABLE entities (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    parent_id   UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    public_key  TEXT NOT NULL,
    private_key TEXT,
    enabled     BOOLEAN NOT NULL DEFAULT TRUE,
    roles       BIGINT[] NOT NULL DEFAULT '{}',
    created_id  UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE
);

CREATE INDEX entities_parent_id_idx ON entities(parent_id);

INSERT INTO roles OVERRIDING SYSTEM VALUE
VALUES (
    0,
    'Full access to ALL',
    'FullTree',
    DEFAULT,
    '{''*''}',
    '{''*''}'
);

-- Seed the tree root. parent_id = id satisfies the NOT NULL FK self-referentially.
INSERT INTO entities (id, parent_id, public_key, private_key, enabled, roles, created_id)
VALUES (
    '01a01500-98d5-71fc-9038-71acb78d61c4',
    '01a01500-98d5-71fc-9038-71acb78d61c4',
    'MCowBQYDK2VwAyEAFZAjhKPyVYKSNK153w7zgoLWhNt9AQJqR7asrpMUil8=',
    'MC4CAQAwBQYDK2VwBCIEIBciTyz1f9ELrN3rZ+tcxxvQa14krR0sxY6HTJOLcWbK',
    TRUE,
    '{0}',
    '01a01500-98d5-71fc-9038-71acb78d61c4'
);
