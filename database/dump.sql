create DATABASE if not exists lunch_and_learn character set utf8 collate utf8_general_ci;

use lunch_and_learn;
create table IF NOT EXISTS `users`
	(id int AUTO_INCREMENT PRIMARY KEY,
    fullname varchar(128) NOT NULL) 
    ENGINE=MyISAM
    CHARACTER SET=utf8 COLLATE utf8_general_ci;
    
insert into `users` (fullname) values
	('Jonathan'),
    ('Michaël'),
    ('Carl'),
    ('Bernard'),
    ('Rose'),
    ('Vincent')