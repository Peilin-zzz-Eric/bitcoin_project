/*
 Navicat Premium Dump SQL

 Source Server         : MySQL
 Source Server Type    : MySQL
 Source Server Version : 80039 (8.0.39)
 Source Host           : localhost:3306
 Source Schema         : block_chain

 Target Server Type    : MySQL
 Target Server Version : 80039 (8.0.39)
 File Encoding         : 65001

 Date: 27/10/2024 20:38:38
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for offchain_metric
-- ----------------------------
DROP TABLE IF EXISTS `offchain_metric`;
CREATE TABLE `offchain_metric`  (
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
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

SET FOREIGN_KEY_CHECKS = 1;
