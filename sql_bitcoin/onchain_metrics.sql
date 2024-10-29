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

 Date: 27/10/2024 20:38:45
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for onchain_metrics
-- ----------------------------
DROP TABLE IF EXISTS `onchain_metrics`;
CREATE TABLE `onchain_metrics`  (
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
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

SET FOREIGN_KEY_CHECKS = 1;
