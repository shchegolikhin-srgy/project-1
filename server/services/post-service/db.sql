CREATE TABLE IF NOT EXISTS posts(
    user_id UUID,
    post_id UUID,
    created_at TIMEUUID,
    title TEXT,
    content TEXT,
    comments_count INT,
    likes_count INT,
    forward_count INT,
    media LIST<TEXT>,
    hashtag SET<TEXT>,
    PRIMARY KEY ((user_id), created_at,post_id)
) WITH CLUSTERING ORDER BY (created_at DESC);

CREATE TABLE IF NOT EXISTS post_likes_by_user(
    user_id UUID,
    post_id UUID,
    
)