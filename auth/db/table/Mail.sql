CREATE TABLE `authMail` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`usr` VARBINARY(255) NOT NULL,`hostId` BIGINT UNSIGNED NOT NULL,PRIMARY KEY (`id`),UNIQUE KEY `unqiue` (`hostId`,`usr`)
);