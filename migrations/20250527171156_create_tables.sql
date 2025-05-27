CREATE TABLE todos (
                       id UUID PRIMARY KEY,
                       title TEXT NOT NULL,
                       completed BOOLEAN NOT NULL DEFAULT FALSE,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE jobs (
                      id UUID PRIMARY KEY,
                      todo_id UUID,
                      operation TEXT NOT NULL,
                      status TEXT NOT NULL,
                      created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                      completed_at TIMESTAMPTZ
);
