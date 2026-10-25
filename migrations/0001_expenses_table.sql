CREATE TABLE IF NOT EXISTS ordered_list (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    title varchar(255) NOT NULL ,
    min_length INT NOT NULL ,
    max_length INT NOT NULL ,
    allowed_types ENUM('ARTICLE') NOT NULL

);