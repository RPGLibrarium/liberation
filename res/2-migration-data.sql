-- MySQL dump 10.17  Distrib 10.3.13-MariaDB, for Linux (x86_64)
--
-- Host: 172.18.0.2    Database: tiki
-- ------------------------------------------------------
-- Server version	10.2.14-MariaDB-10.2.14+maria~jessie

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `books`
--

DROP TABLE IF EXISTS `books`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `books` (
  `book_id` int(11) NOT NULL AUTO_INCREMENT,
  `title_by_id` int(11) NOT NULL,
  `owner_member_by_id` int(11) DEFAULT NULL,
  `owner_guild_by_id` int(11) DEFAULT NULL,
  `owner_type` enum('guild','member') GENERATED ALWAYS AS (if(`owner_guild_by_id` is not null,'guild','member')) STORED,
  `quality` text NOT NULL,
  `external_inventory_id` int(11) NOT NULL,
  PRIMARY KEY (`book_id`),
  UNIQUE KEY `external_inventory_id` (`external_inventory_id`),
  KEY `title_by_id` (`title_by_id`),
  KEY `owner_member_by_id` (`owner_member_by_id`),
  KEY `owner_guild_by_id` (`owner_guild_by_id`),
  CONSTRAINT `books_ibfk_1` FOREIGN KEY (`title_by_id`) REFERENCES `titles` (`title_id`),
  CONSTRAINT `books_ibfk_2` FOREIGN KEY (`owner_member_by_id`) REFERENCES `members` (`member_id`),
  CONSTRAINT `books_ibfk_3` FOREIGN KEY (`owner_guild_by_id`) REFERENCES `guilds` (`guild_id`),
  CONSTRAINT `CONSTRAINT_1` CHECK (`owner_guild_by_id` is not null xor `owner_member_by_id` is not null)
) ENGINE=InnoDB AUTO_INCREMENT=202 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `books`
--

LOCK TABLES `books` WRITE;
/*!40000 ALTER TABLE `books` DISABLE KEYS */;
INSERT INTO `books` VALUES (9,13,NULL,1,'guild','',2),(10,14,NULL,1,'guild','',3),(11,17,NULL,1,'guild','',1),(32,15,NULL,1,'guild','',4),(33,16,NULL,1,'guild','',5),(34,18,NULL,1,'guild','',6),(35,19,NULL,1,'guild','',7),(36,21,NULL,1,'guild','',9),(37,22,NULL,1,'guild','',10),(38,23,NULL,1,'guild','',11),(39,24,NULL,1,'guild','',12),(40,25,NULL,1,'guild','',13),(41,26,NULL,1,'guild','',14),(42,27,NULL,1,'guild','',15),(43,28,NULL,1,'guild','',16),(44,29,NULL,1,'guild','',17),(45,30,NULL,1,'guild','',18),(46,31,NULL,1,'guild','',19),(47,20,NULL,1,'guild','',8),(66,60,NULL,1,'guild','',20),(67,61,NULL,1,'guild','',21),(68,62,NULL,1,'guild','',22),(69,63,NULL,1,'guild','',23),(70,64,NULL,1,'guild','',24),(71,65,NULL,1,'guild','',25),(73,72,NULL,1,'guild','',26),(75,74,NULL,1,'guild','',27),(77,76,NULL,1,'guild','',28),(79,78,NULL,1,'guild','',29),(81,80,NULL,1,'guild','',30),(83,82,NULL,1,'guild','',32),(85,84,NULL,1,'guild','',34),(87,86,NULL,1,'guild','',33),(89,88,NULL,1,'guild','',31),(96,95,NULL,1,'guild','',35),(106,105,NULL,1,'guild','',36),(108,107,NULL,1,'guild','',37),(110,109,NULL,1,'guild','',38),(112,111,NULL,1,'guild','',39),(114,113,NULL,1,'guild','',40),(124,119,NULL,1,'guild','',41),(125,120,NULL,1,'guild','',42),(126,121,NULL,1,'guild','',43),(127,123,NULL,1,'guild','',44),(128,122,NULL,1,'guild','',45),(131,129,NULL,1,'guild','',46),(132,130,NULL,1,'guild','',47),(137,133,NULL,1,'guild','',48),(138,136,NULL,1,'guild','',49),(139,135,NULL,1,'guild','',50),(140,134,NULL,1,'guild','',51),(144,141,NULL,1,'guild','',53),(145,142,NULL,1,'guild','',54),(146,143,NULL,1,'guild','',55),(148,147,NULL,1,'guild','',52),(151,149,NULL,1,'guild','',56),(152,150,NULL,1,'guild','',57),(154,153,NULL,1,'guild','',58),(156,155,NULL,1,'guild','',59),(159,157,NULL,1,'guild','',60),(160,158,NULL,1,'guild','',61),(162,161,NULL,1,'guild','',62),(164,163,NULL,1,'guild','',63),(186,185,NULL,1,'guild','',64),(197,192,NULL,1,'guild','',65),(198,193,NULL,1,'guild','',66),(199,194,NULL,1,'guild','',67),(200,195,NULL,1,'guild','',68),(201,196,NULL,1,'guild','',69);
/*!40000 ALTER TABLE `books` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `guilds`
--

DROP TABLE IF EXISTS `guilds`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `guilds` (
  `guild_id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `address` text NOT NULL,
  `contact_by_member_id` int(11) NOT NULL,
  PRIMARY KEY (`guild_id`),
  UNIQUE KEY `name` (`name`),
  KEY `contact_by_member_id` (`contact_by_member_id`),
  CONSTRAINT `guilds_ibfk_1` FOREIGN KEY (`contact_by_member_id`) REFERENCES `members` (`member_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `guilds`
--

LOCK TABLES `guilds` WRITE;
/*!40000 ALTER TABLE `guilds` DISABLE KEYS */;
INSERT INTO `guilds` VALUES (1,'RPG Librarium Aachen','Schurzelter Str. 469, 52074 Aachen',999);
/*!40000 ALTER TABLE `guilds` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `members`
--

DROP TABLE IF EXISTS `members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `members` (
  `member_id` int(11) NOT NULL AUTO_INCREMENT,
  `external_id` varchar(255) NOT NULL,
  PRIMARY KEY (`member_id`),
  UNIQUE KEY `external_id` (`external_id`)
) ENGINE=InnoDB AUTO_INCREMENT=1000 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `members`
--

LOCK TABLES `members` WRITE;
/*!40000 ALTER TABLE `members` DISABLE KEYS */;
INSERT INTO `members` VALUES (999,'REPLACE HERE');
/*!40000 ALTER TABLE `members` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `rentals`
--

DROP TABLE IF EXISTS `rentals`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `rentals` (
  `rental_id` int(11) NOT NULL AUTO_INCREMENT,
  `from_date` date NOT NULL,
  `to_date` date NOT NULL,
  `book_by_id` int(11) NOT NULL,
  `rentee_member_by_id` int(11) DEFAULT NULL,
  `rentee_guild_by_id` int(11) DEFAULT NULL,
  `rentee_type` enum('guild','member') GENERATED ALWAYS AS (if(`rentee_guild_by_id` is not null,'guild','member')) STORED,
  PRIMARY KEY (`rental_id`),
  KEY `book_by_id` (`book_by_id`),
  KEY `rentee_member_by_id` (`rentee_member_by_id`),
  KEY `rentee_guild_by_id` (`rentee_guild_by_id`),
  CONSTRAINT `rentals_ibfk_1` FOREIGN KEY (`book_by_id`) REFERENCES `books` (`book_id`),
  CONSTRAINT `rentals_ibfk_2` FOREIGN KEY (`rentee_member_by_id`) REFERENCES `members` (`member_id`),
  CONSTRAINT `rentals_ibfk_3` FOREIGN KEY (`rentee_guild_by_id`) REFERENCES `guilds` (`guild_id`),
  CONSTRAINT `CONSTRAINT_1` CHECK (`rentee_guild_by_id` is not null xor `rentee_member_by_id` is not null)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `rentals`
--

LOCK TABLES `rentals` WRITE;
/*!40000 ALTER TABLE `rentals` DISABLE KEYS */;
/*!40000 ALTER TABLE `rentals` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `rpg_systems`
--

DROP TABLE IF EXISTS `rpg_systems`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `rpg_systems` (
  `rpg_system_id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `shortname` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`rpg_system_id`),
  UNIQUE KEY `name` (`name`),
  UNIQUE KEY `shortname` (`shortname`)
) ENGINE=InnoDB AUTO_INCREMENT=32 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `rpg_systems`
--

LOCK TABLES `rpg_systems` WRITE;
/*!40000 ALTER TABLE `rpg_systems` DISABLE KEYS */;
INSERT INTO `rpg_systems` VALUES (2,'Das Schwarze Auge 4.1 Edition','DSA 4.1'),(3,'Shadowrun 5',NULL),(4,'Hollow Earth',NULL),(6,'Cthuhlu 7. Ed. ',NULL),(8,'Dungeons & Dragons 5. Edition','D&D 5'),(9,'World of Darkness',NULL),(10,'Generisches System mit vielen Setting Büchern','Savage Worlds'),(16,'Das Schwarze Auge 5. Edition','DSA 5'),(17,'Fragged Empire',NULL),(18,'Einzelstücke',NULL),(19,'Paranoia',NULL),(20,'Mutants & Masterminds 3.Ed.',NULL),(21,'1W6 Freunde',NULL),(22,'Splittermond',NULL),(23,'7te See',NULL),(24,'Sekundärliteratur','Meta'),(25,'Legend of the Five Rings 2. Ed.',NULL),(26,'PDQ#',NULL),(27,'D&D 3',NULL);
/*!40000 ALTER TABLE `rpg_systems` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `titles`
--

DROP TABLE IF EXISTS `titles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `titles` (
  `title_id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `rpg_system_by_id` int(11) NOT NULL,
  `language` varchar(255) NOT NULL,
  `publisher` varchar(255) NOT NULL,
  `year` smallint(6) NOT NULL,
  `coverimage` text DEFAULT NULL,
  PRIMARY KEY (`title_id`),
  UNIQUE KEY `name` (`name`),
  KEY `rpg_system_by_id` (`rpg_system_by_id`),
  CONSTRAINT `titles_ibfk_1` FOREIGN KEY (`rpg_system_by_id`) REFERENCES `rpg_systems` (`rpg_system_id`)
) ENGINE=InnoDB AUTO_INCREMENT=197 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `titles`
--

LOCK TABLES `titles` WRITE;
/*!40000 ALTER TABLE `titles` DISABLE KEYS */;
INSERT INTO `titles` VALUES (13,'Wege der Helden',2,'de','',0,NULL),(14,'Plüsch, Power & Plunder Basisregelwerk',18,'de','',0,NULL),(15,'Die Priester der Krähen',6,'de','',0,NULL),(16,'Straßengrimoire',3,'de','',0,NULL),(17,'17Grundregelwerk',3,'de','',0,NULL),(18,'Schattenhandbuch 2',3,'de','',0,NULL),(19,'Kreuzfeuer',3,'de','',0,NULL),(20,'Hollow Earth Expedition',4,'de','',0,NULL),(21,'All Flesh Must Be Eaten',18,'de','',0,NULL),(22,'Herz des Reiches',2,'de','',0,NULL),(23,'Schild des Reiches',2,'de','',0,NULL),(24,'Klingentänzer',2,'de','',0,NULL),(25,'Hallen Arkaner Macht',2,'de','',0,NULL),(26,'Bodyshop',3,'de','',0,NULL),(27,'Datenpfade',3,'de','',0,NULL),(28,'Elementare Gewalten',2,'de','',0,NULL),(29,'In den Dschungeln Meridiana',2,'de','',0,NULL),(30,'Horte Magischen Wissens',2,'de','',0,NULL),(31,'Von Toten und Untoten',2,'de','',0,NULL),(60,'Efferds Wogen',2,'de','',0,NULL),(61,'Player\'s Handbook',8,'de','',0,NULL),(62,'Dungeon Master\'s Guide',8,'de','',0,NULL),(63,'Monster Manual',8,'de','',0,NULL),(64,'Jubiläumsausgabe Vampire die Masquerade',9,'de','',0,NULL),(65,'Hellfrost Spielerhandbuch',10,'de','',0,NULL),(72,'Das Schwarze Auge Regelwerk',16,'de','',0,NULL),(74,'74Grundregelwerk',17,'de','',0,NULL),(76,'Machoweiber mit dicken Kanonen',18,'de','',0,NULL),(78,'Gentleman\'s Edition Revised',10,'de','',0,NULL),(80,'Investigatoren-Kompendium',6,'de','',0,NULL),(82,'Kobolde fressen Babys!',18,'de','',0,NULL),(84,'Ars Mathematica',6,'de','',0,NULL),(86,'Tales from the Yawning Portal',8,'de','',0,NULL),(88,'Seattle Box',3,'de','',0,NULL),(95,'Paranoia - Red Clearance Edition (Box)',19,'de','',0,NULL),(105,'Sea Dracula',18,'de','',0,NULL),(107,'Der Kult der goldenen Masken',2,'de','',0,NULL),(109,'Rippers-Resurrected Spielerhandbuch',10,'de','',0,NULL),(111,'Rippers-Resurrected Spielleiterhandbuch',10,'de','',0,NULL),(113,'Deluxe Hero\'s Handbook',20,'de','',0,NULL),(119,'Schrecken aus der Tiefe',18,'de','',0,NULL),(120,'Zweite Edition',21,'de','',0,NULL),(121,'Geister, Gauner und Halunken',21,'de','',0,NULL),(122,'Der Almanach zum Gratisrollenspieltag 2018',18,'de','',0,NULL),(123,'123Grundregelwerk',6,'de','',0,NULL),(129,'Verschworene Gemeinschaften',2,'de','',0,NULL),(130,'Orden und Bündnisse',2,'de','',0,NULL),(133,'Stätten Okkulter Geheimnisse',2,'de','',0,NULL),(134,'Aventurischer Atlas',2,'de','',0,NULL),(135,'Die Reisende Kaiserin',2,'de','',0,NULL),(136,'Schattenlande',2,'de','',0,NULL),(141,'Peraine-Vademecum',2,'de','',0,NULL),(142,'Travia-Vademecum',2,'de','',0,NULL),(143,'Rahjasutra',2,'de','',0,NULL),(147,'Schattenhandbuch 3',3,'de','',0,NULL),(149,'Die Regeln',22,'de','',0,NULL),(150,'Die Welt',22,'de','',0,NULL),(153,'Mondstahlklingen',22,'de','',0,NULL),(155,'Asphaltkrieger',3,'de','',0,NULL),(157,'157Grundregelwerk',23,'de','',0,NULL),(158,'Spielleiterschirm',23,'de','',0,NULL),(161,'Ratten - Revised',18,'de','',0,NULL),(163,'Wege des Schwerts',2,'de','',0,NULL),(185,'Die Magie',22,'de','',0,NULL),(192,'Player\'s Guide',25,'de','',0,NULL),(193,'Sword & Sorcery - Kreaturen Kompendium',27,'de','',0,NULL),(194,'Swashbucklers of the 7 Skies',26,'de','',0,NULL),(195,'imporv for gamers',24,'de','',0,NULL),(196,'#Feminism - A Nano-Game Anthology',18,'de','',0,NULL);
/*!40000 ALTER TABLE `titles` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2019-03-30 17:38:47
