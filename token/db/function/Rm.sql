CREATE FUNCTION `tokenRm`(`uid` BIGINT UNSIGNED,`id` BIGINT UNSIGNED) RETURNS TINYINT
BEGIN
    DECLARE r TINYINT UNSIGNED;
    DELETE FROM token WHERE token.id=id AND token.uid=uid;
    SET r = ROW_COUNT();
    RETURN r > 0;
END ;;