
DROP TABLE IF EXISTS `offchain_metric`;
CREATE TABLE `offchain_metric` (
  `market_cap` bigint NOT NULL,
  `diluted_market_cap` bigint NOT NULL,
  `volume_24h` bigint NOT NULL,
  `vol_market_cap_ratio` decimal(10, 8) NOT NULL,
  `low_24h` decimal(12, 2) NOT NULL,
  `high_24h` decimal(12, 2) NOT NULL,
  `price_change_24h` decimal(10, 4) NOT NULL,
  `circulating_supply` decimal(18, 8) NOT NULL,
  `max_supply` decimal(18, 8) NOT NULL,
  `genesis_block_date` date NOT NULL,
  `last_updated` timestamp NULL DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS `onchain_metrics`;
CREATE TABLE `onchain_metrics` (
  `block_height` bigint NOT NULL,
  `block_hash` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `block_version` int NULL DEFAULT NULL,
  `previous_block_hash` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `timestamp` timestamp NULL DEFAULT NULL,
  `confirmations` bigint NULL DEFAULT NULL,
  `block_size` bigint NULL DEFAULT NULL,
  `block_weight` bigint NULL DEFAULT NULL,
  `difficulty` double NULL DEFAULT NULL,
  `transaction_count` bigint NULL DEFAULT NULL,
  `avg_tx_size` double NULL DEFAULT NULL,
  `total_supply` double NULL DEFAULT NULL,
  `block_time` double NULL DEFAULT NULL,
  `block_interval` double NULL DEFAULT NULL,
  `hash_rate` double NULL DEFAULT NULL,
  `nonce` bigint UNSIGNED NULL DEFAULT NULL,
  `merkle_root` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  PRIMARY KEY (`block_height`) USING BTREE
);

DROP TABLE IF EXISTS `price_day`;
CREATE TABLE `price_day` (
  `id` int NOT NULL AUTO_INCREMENT,
  `timestamp` datetime NOT NULL,
  `price` decimal(20, 8) NOT NULL,
  `market_cap` bigint NOT NULL,
  `volume` bigint NOT NULL,
  PRIMARY KEY (`id`) USING BTREE
);

DROP TABLE IF EXISTS `price_month`;
CREATE TABLE `price_month` (
  `id` int NOT NULL AUTO_INCREMENT,
  `timestamp` datetime NOT NULL,
  `price` decimal(20, 8) NOT NULL,
  `market_cap` bigint NOT NULL,
  `volume` bigint NOT NULL,
  PRIMARY KEY (`id`) USING BTREE
);

DROP TABLE IF EXISTS `price_week`;
CREATE TABLE `price_week` (
  `id` int NOT NULL AUTO_INCREMENT,
  `timestamp` datetime NOT NULL,
  `price` decimal(20, 8) NOT NULL,
  `market_cap` bigint NOT NULL,
  `volume` bigint NOT NULL,
  PRIMARY KEY (`id`) USING BTREE
);

DROP TABLE IF EXISTS `price_year`;
CREATE TABLE `price_year` (
  `id` int NOT NULL AUTO_INCREMENT,
  `timestamp` datetime NOT NULL,
  `price` decimal(20, 8) NOT NULL,
  `market_cap` bigint NOT NULL,
  `volume` bigint NOT NULL,
  PRIMARY KEY (`id`) USING BTREE
);
