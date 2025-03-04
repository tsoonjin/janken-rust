ALTER TABLE players
ADD COLUMN wins INTEGER DEFAULT 0,
ADD COLUMN losses INTEGER DEFAULT 0,
ADD COLUMN draws INTEGER DEFAULT 0;

CREATE TABLE match_history (
    id UUID PRIMARY KEY,
    player1_id UUID REFERENCES players(id),
    player2_id UUID REFERENCES players(id),
    winner_id UUID REFERENCES players(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);