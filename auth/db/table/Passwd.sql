CREATE TABLE `authPasswd` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`hash` BINARY(16) NOT NULL,`ts` BIGINT UNSIGNED NOT NULL,PRIMARY KEY (`id`)
);