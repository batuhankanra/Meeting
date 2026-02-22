-- UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- USERS
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

-- MEETINGS
CREATE TABLE meetings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    host_id UUID NOT NULL,
    scheduled_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),

    CONSTRAINT fk_host
        FOREIGN KEY (host_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

-- MEETING PARTICIPANTS (katılanlar)
CREATE TABLE meeting_participants (
    meeting_id UUID NOT NULL,
    user_id UUID NOT NULL,
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT now(),

    PRIMARY KEY (meeting_id, user_id),

    CONSTRAINT fk_meeting
        FOREIGN KEY (meeting_id)
        REFERENCES meetings(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

-- MEETING INVITES (davetliler)
CREATE TABLE meeting_invites (
    meeting_id UUID NOT NULL,
    user_id UUID NOT NULL,
    invited_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    invited_by UUID,

    PRIMARY KEY (meeting_id, user_id),

    CONSTRAINT fk_invite_meeting
        FOREIGN KEY (meeting_id)
        REFERENCES meetings(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_invite_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_invited_by
        FOREIGN KEY (invited_by)
        REFERENCES users(id)
        ON DELETE SET NULL
);

-- INDEXLER
CREATE INDEX idx_invites_user_id ON meeting_invites(user_id);
CREATE INDEX idx_meetings_host_id ON meetings(host_id);
CREATE INDEX idx_participants_user_id ON meeting_participants(user_id);