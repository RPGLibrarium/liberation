-- phpMyAdmin SQL Dump
-- version 4.6.6deb4
-- https://www.phpmyadmin.net/
--
-- Host: localhost:3306
-- Generation Time: Mar 30, 2019 at 04:07 PM
-- Server version: 10.1.37-MariaDB-0+deb9u1
-- PHP Version: 5.6.33-0+deb8u1

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `tikiwiki`
--

-- --------------------------------------------------------

--
-- Table structure for table `tiki_trackers`
--

CREATE TABLE `tiki_trackers` (
  `trackerId` int(12) NOT NULL,
  `name` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  `description` text COLLATE utf8_unicode_ci,
  `descriptionIsParsed` varchar(1) COLLATE utf8_unicode_ci DEFAULT '0',
  `created` int(14) DEFAULT NULL,
  `lastModif` int(14) DEFAULT NULL,
  `items` int(10) DEFAULT NULL
) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `tiki_trackers`
--

INSERT INTO `tiki_trackers` (`trackerId`, `name`, `description`, `descriptionIsParsed`, `created`, `lastModif`, `items`) VALUES
(2, 'Buch', '', 'y', 1464353024, 1552815431, 69),
(3, 'Titel', '', 'n', 1464357927, 1547371999, 69),
(4, 'Warteliste', '', 'n', 1464505996, 1464516371, 0),
(6, 'Anmeldungen', '', 'n', 1466754810, 1544957061, 23);

-- --------------------------------------------------------

--
-- Table structure for table `tiki_tracker_fields`
--

CREATE TABLE `tiki_tracker_fields` (
  `fieldId` int(12) NOT NULL,
  `trackerId` int(12) NOT NULL DEFAULT '0',
  `name` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  `permName` varchar(100) COLLATE utf8_unicode_ci DEFAULT NULL,
  `options` text COLLATE utf8_unicode_ci,
  `type` varchar(15) COLLATE utf8_unicode_ci DEFAULT NULL,
  `isMain` char(1) COLLATE utf8_unicode_ci DEFAULT NULL,
  `isTblVisible` char(1) COLLATE utf8_unicode_ci DEFAULT NULL,
  `position` int(4) DEFAULT NULL,
  `isSearchable` char(1) COLLATE utf8_unicode_ci NOT NULL DEFAULT 'y',
  `isPublic` char(1) COLLATE utf8_unicode_ci NOT NULL DEFAULT 'n',
  `isHidden` char(1) COLLATE utf8_unicode_ci NOT NULL DEFAULT 'n',
  `isMandatory` char(1) COLLATE utf8_unicode_ci NOT NULL DEFAULT 'n',
  `description` text COLLATE utf8_unicode_ci,
  `isMultilingual` char(1) COLLATE utf8_unicode_ci DEFAULT 'n',
  `itemChoices` text COLLATE utf8_unicode_ci,
  `errorMsg` text COLLATE utf8_unicode_ci,
  `visibleBy` text COLLATE utf8_unicode_ci,
  `editableBy` text COLLATE utf8_unicode_ci,
  `descriptionIsParsed` char(1) COLLATE utf8_unicode_ci DEFAULT 'n',
  `validation` varchar(255) COLLATE utf8_unicode_ci DEFAULT '',
  `validationParam` varchar(255) COLLATE utf8_unicode_ci DEFAULT '',
  `validationMessage` varchar(255) COLLATE utf8_unicode_ci DEFAULT ''
) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `tiki_tracker_fields`
--

INSERT INTO `tiki_tracker_fields` (`fieldId`, `trackerId`, `name`, `permName`, `options`, `type`, `isMain`, `isTblVisible`, `position`, `isSearchable`, `isPublic`, `isHidden`, `isMandatory`, `description`, `isMultilingual`, `itemChoices`, `errorMsg`, `visibleBy`, `editableBy`, `descriptionIsParsed`, `validation`, `validationParam`, `validationMessage`) VALUES
(12, 2, 'Gebraucht bis', 'gebrauchtBis', '{\"datetime\":\"dt\",\"useNow\":0}', 'j', 'y', 'y', 60, 'n', 'y', 'n', 'n', '', 'n', '', '', '', '', 'n', '', '', ''),
(10, 2, 'Besitzer', 'besitzer', '{\"autoassign\":2,\"owner\":0,\"notify\":0,\"notify_template\":\"\",\"notify_template_format\":\"text\",\"multiple\":0,\"groupIds\":[6],\"canChangeGroupIds\":[6,2],\"showRealname\":1}', 'u', 'y', 'y', 40, 'n', 'y', 'n', 'n', '', 'n', '', '', 'a:2:{i:0;s:10:\"Registered\";i:1;s:8:\"Mitglied\";}', '', 'n', '', '', ''),
(11, 2, 'Ausleihungsdatum', 'ausleihungsdatum', '{\"datetime\":\"dt\",\"useNow\":0}', 'j', 'y', 'y', 50, 'n', 'y', 'n', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(9, 2, 'Eigentuemer', 'eigentuemer', '{\"autoassign\":1,\"owner\":0,\"notify\":2,\"notify_template\":\"\",\"notify_template_format\":\"text\",\"multiple\":0,\"groupIds\":[0],\"canChangeGroupIds\":[0],\"showRealname\":1}', 'u', 'y', 'y', 30, 'n', 'y', 'p', 'n', '', 'n', '', '', 'a:2:{i:0;s:8:\"Mitglied\";i:1;s:10:\"Registered\";}', '', 'n', '', '', ''),
(20, 2, 'Nr.', 'inventarnummer', '{\"samerow\":1,\"size\":5,\"prepend\":\"\",\"append\":\"\",\"decimals\":0,\"dec_point\":\".\",\"thousands\":\",\"}', 'n', 'y', 'y', 0, 'n', 'y', 'p', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(24, 4, 'Datum', 'datum', '', 'j', 'n', 'n', 30, 'n', 'y', 'n', 'n', '', '', '', '', '', '', 'n', '', '', ''),
(13, 2, 'Qualitaet', 'qualitaet', '{\"options\":[\"0=kaputt\",\" 1=beschaedigt\",\" 2=gebraucht\",\" 3=unbeschaedigt\"]}', 'd', 'y', 'y', 70, 'n', 'y', 'n', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(19, 2, 'Titel', 'titel', '{\"trackerId\":3,\"fieldId\":16,\"linkToItem\":0,\"displayFieldsList\":[16,18],\"status\":\"opc\",\"linkPage\":\"\",\"addItems\":\"\",\"addItemsWikiTpl\":\"\",\"preSelectFieldHere\":0,\"preSelectFieldThere\":0,\"preSelectFieldMethod\":\"exact\",\"displayOneItem\":\"multi\",\"selectMultipleValues\":0,\"indexRemote\":[16,18],\"cascade\":0}', 'r', 'y', 'y', 10, 'n', 'y', 'p', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(16, 3, 'Name', 'name', '{\"samerow\":0,\"size\":0,\"prepend\":\"\",\"append\":\"\",\"max\":0,\"autocomplete\":\"n\",\"exact\":\"n\"}', 't', 'y', 'y', 10, 'y', 'y', 'n', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(18, 3, 'System', 'system', '{\"parentId\":1,\"inputtype\":\"d\",\"selectall\":0,\"descendants\":0,\"help\":0,\"outputtype\":\"\",\"doNotInheritCategories\":0,\"recategorize\":\"save\"}', 'e', 'y', 'y', 30, 'y', 'y', 'n', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(21, 2, 'Auflage', 'auflage', '{\"samerow\":1,\"size\":2,\"prepend\":\"\",\"append\":\"\",\"decimals\":0,\"dec_point\":\".\",\"thousands\":\",\"}', 'n', 'y', 'y', 20, 'n', 'y', 'p', 'n', '', 'n', '', '', '', '', 'n', '', '', ''),
(22, 4, 'Buch', 'buch', '{\"trackerId\":2,\"fieldId\":20,\"linkToItem\":0,\"displayFieldsList\":[20,19],\"status\":\"opc\",\"linkPage\":\"\",\"addItems\":\"\",\"addItemsWikiTpl\":\"\",\"preSelectFieldHere\":0,\"preSelectFieldThere\":0,\"preSelectFieldMethod\":\"exact\",\"displayOneItem\":\"multi\",\"selectMultipleValues\":0,\"indexRemote\":[0],\"cascade\":0}', 'r', 'y', 'y', 10, 'n', 'y', 'n', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(23, 4, 'Person', 'person', '{\"autoassign\":1,\"notify\":0,\"groupIds\":[0],\"canChangeGroupIds\":[0],\"showRealname\":1}', 'u', 'y', 'y', 20, 'n', 'y', 'n', 'y', '', 'n', '', '', '', '', 'n', '', '', ''),
(29, 6, 'Alter', 'alter', '{\"options\":[\"17 und j\\u00fcnger\",\"18-26\",\" 27 und \\u00e4lter\"]}', 'd', 'n', 'y', 20, 'n', 'y', 'n', 'y', 'zum Zeitpunkt der Freizeit', 'n', '', '', '', '', 'n', '', '', ''),
(27, 3, 'Seite', 'seite', '{\"linkToURL\":4,\"other\":\"Details\"}', 'L', 'n', 'y', 40, 'n', 'y', 'n', 'n', '', 'n', '', '', '', '', 'n', '', '', ''),
(28, 6, 'Name', 'name', '', 't', 'y', 'y', 10, 'n', 'y', 'n', 'y', 'Voller Name (Vor- und Nachname)', '', '', '', '', '', 'n', '', '', ''),
(30, 6, 'Anschrift', 'anschrift', '', 'a', 'n', 'y', 30, 'n', 'y', 'n', 'y', '', '', '', '', '', '', 'n', '', '', ''),
(31, 6, 'E-Mail Adresse', 'eMailAdresse', '', 't', 'n', 'y', 40, 'n', 'y', 'n', 'y', 'unter der du erreichbar bist. (Es gibt auch keinen Spam)', '', '', '', '', '', 'n', '', '', ''),
(33, 6, 'Nahrungsrestriktionen', 'nahrungsrestriktionen', '{\"samerow\":1,\"size\":0,\"prepend\":\"\",\"append\":\"\",\"max\":0,\"autocomplete\":\"n\",\"exact\":\"n\"}', 't', 'n', 'y', 50, 'n', 'y', 'n', 'n', 'Vegetarisch, Vegan, Intoleranzen, Allergien etc.', 'n', '', '', '', '', 'n', '', '', ''),
(34, 6, 'An- und Abreise', 'anUndAbreise', '{\"samerow\":1,\"size\":0,\"prepend\":\"\",\"append\":\"\",\"max\":0,\"autocomplete\":\"n\",\"exact\":\"n\"}', 't', 'n', 'y', 60, 'n', 'y', 'n', 'n', 'Wenn du später anreisen oder früher abreisen möchtest, bitte hier angeben.', 'n', '', '', '', '', 'n', '', '', '');

-- --------------------------------------------------------

--
-- Table structure for table `tiki_tracker_items`
--

CREATE TABLE `tiki_tracker_items` (
  `itemId` int(12) NOT NULL,
  `trackerId` int(12) NOT NULL DEFAULT '0',
  `created` int(14) DEFAULT NULL,
  `createdBy` varchar(200) COLLATE utf8_unicode_ci DEFAULT NULL,
  `status` char(1) COLLATE utf8_unicode_ci DEFAULT NULL,
  `lastModif` int(14) DEFAULT NULL,
  `lastModifBy` varchar(200) COLLATE utf8_unicode_ci DEFAULT NULL
) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Dumping data for table `tiki_tracker_items`
--

INSERT INTO `tiki_tracker_items` (`itemId`, `trackerId`, `created`, `createdBy`, `status`, `lastModif`, `lastModifBy`) VALUES
(14, 3, 1464444978, 'tkehler', 'o', 1500206430, 'tkehler'),
(13, 3, 1464358349, 'tkehler', 'o', 1473275753, 'tkehler'),
(9, 2, 1464353807, 'tkehler', 'o', 1547370621, 'cbeck'),
(10, 2, 1464356757, 'tkehler', 'o', 1546934769, 'hfranzen'),
(11, 2, 1464356827, 'tkehler', 'o', 1547369310, 'tkehler'),
(15, 3, 1464445015, 'tkehler', 'o', 1473275598, 'tkehler'),
(16, 3, 1464445038, 'tkehler', 'o', 1473275730, 'tkehler'),
(17, 3, 1464445180, 'tkehler', 'o', 1473276965, 'tkehler'),
(18, 3, 1464445283, 'tkehler', 'o', 1473275707, 'tkehler'),
(19, 3, 1464445303, 'tkehler', 'o', 1473275668, 'tkehler'),
(20, 3, 1464445332, 'tkehler', 'o', 1473275617, 'tkehler'),
(21, 3, 1464445349, 'tkehler', 'o', 1513620600, 'tkehler'),
(22, 3, 1464445361, 'tkehler', 'o', 1466189349, 'tkehler'),
(23, 3, 1464445372, 'tkehler', 'o', 1473275718, 'tkehler'),
(24, 3, 1464445383, 'tkehler', 'o', 1473275657, 'tkehler'),
(25, 3, 1464445392, 'tkehler', 'o', 1466189363, 'tkehler'),
(26, 3, 1464445402, 'tkehler', 'o', 1473275513, 'tkehler'),
(27, 3, 1464445413, 'tkehler', 'o', 1473275507, 'tkehler'),
(28, 3, 1464445427, 'tkehler', 'o', 1473275150, 'tkehler'),
(29, 3, 1464445443, 'tkehler', 'o', 1473275644, 'tkehler'),
(30, 3, 1464445459, 'tkehler', 'o', 1473275627, 'tkehler'),
(31, 3, 1464445474, 'tkehler', 'o', 1473275742, 'tkehler'),
(32, 2, 1464446082, 'tkehler', 'o', 1547370070, 'tkehler'),
(33, 2, 1464446140, 'tkehler', 'o', 1547370189, 'abuescher'),
(34, 2, 1464446186, 'tkehler', 'o', 1547369302, 'tkehler'),
(35, 2, 1464446225, 'tkehler', 'o', 1547370048, 'tkehler'),
(36, 2, 1464446298, 'tkehler', 'o', 1547369175, 'tkehler'),
(37, 2, 1464446374, 'tkehler', 'o', 1547370547, 'cbeck'),
(38, 2, 1464446393, 'tkehler', 'o', 1547370560, 'cbeck'),
(39, 2, 1464446422, 'tkehler', 'o', 1547370535, 'cbeck'),
(40, 2, 1464446467, 'tkehler', 'o', 1547369421, 'tkehler'),
(41, 2, 1464446492, 'tkehler', 'o', 1547369402, 'tkehler'),
(42, 2, 1464446530, 'tkehler', 'o', 1547369440, 'tkehler'),
(43, 2, 1464446554, 'tkehler', 'o', 1547369412, 'tkehler'),
(44, 2, 1464446585, 'tkehler', 'o', 1547370607, 'cbeck'),
(45, 2, 1464446606, 'tkehler', 'o', 1547370576, 'cbeck'),
(46, 2, 1464446623, 'tkehler', 'o', 1547369461, 'tkehler'),
(47, 2, 1464446848, 'tkehler', 'o', 1547369548, 'tkehler'),
(96, 2, 1504174604, 'tkehler', 'o', 1552815431, 'fheiden'),
(95, 3, 1504174556, 'tkehler', 'o', 1530102140, 'tkehler'),
(182, 6, 1540032657, NULL, 'o', 1540032657, NULL),
(179, 6, 1538390794, NULL, 'o', 1538390794, NULL),
(180, 6, 1539269690, NULL, 'o', 1539269690, NULL),
(181, 6, 1539776972, NULL, 'o', 1539776972, NULL),
(60, 3, 1473275823, 'tkehler', 'o', 1473275823, 'tkehler'),
(61, 3, 1473275857, 'tkehler', 'o', 1473275857, 'tkehler'),
(62, 3, 1473275878, 'tkehler', 'o', 1473275878, 'tkehler'),
(63, 3, 1473275895, 'tkehler', 'o', 1473275895, 'tkehler'),
(64, 3, 1473275928, 'tkehler', 'o', 1473275928, 'tkehler'),
(65, 3, 1473275958, 'tkehler', 'o', 1473275958, 'tkehler'),
(66, 2, 1473276019, 'tkehler', 'o', 1552814181, 'paradyx'),
(67, 2, 1473276044, 'tkehler', 'o', 1552815380, 'fheiden'),
(68, 2, 1473276071, 'tkehler', 'o', 1547369284, 'tkehler'),
(69, 2, 1473276107, 'tkehler', 'o', 1547370129, 'lbadaru'),
(70, 2, 1473276145, 'tkehler', 'o', 1547370320, 'fheiden'),
(71, 2, 1473276167, 'tkehler', 'o', 1547370312, 'fheiden'),
(72, 3, 1480159519, 'tkehler', 'o', 1480159519, 'tkehler'),
(73, 2, 1480159573, 'tkehler', 'o', 1547370202, 'abuescher'),
(74, 3, 1495704633, 'tkehler', 'o', 1495704633, 'tkehler'),
(75, 2, 1495704692, 'tkehler', 'o', 1547369470, 'tkehler'),
(76, 3, 1497000311, 'tkehler', 'o', 1497000311, 'tkehler'),
(77, 2, 1497001564, 'tkehler', 'o', 1547368559, 'tkehler'),
(78, 3, 1497013398, 'tkehler', 'o', 1497013398, 'tkehler'),
(79, 2, 1497013443, 'tkehler', 'o', 1547370437, 'hfranzen'),
(80, 3, 1497688330, 'tkehler', 'o', 1497688330, 'tkehler'),
(81, 2, 1497688373, 'tkehler', 'o', 1547370154, 'lbadaru'),
(82, 3, 1498473394, 'tkehler', 'o', 1498473394, 'tkehler'),
(83, 2, 1498473490, 'tkehler', 'o', 1547369430, 'tkehler'),
(84, 3, 1498473593, 'tkehler', 'o', 1498473593, 'tkehler'),
(85, 2, 1498473663, 'tkehler', 'o', 1547370138, 'lbadaru'),
(86, 3, 1498473699, 'tkehler', 'o', 1498473699, 'tkehler'),
(87, 2, 1498473750, 'tkehler', 'o', 1547370118, 'lbadaru'),
(88, 3, 1498473950, 'tkehler', 'o', 1498473950, 'tkehler'),
(89, 2, 1498474037, 'tkehler', 'o', 1547370329, 'fheiden'),
(176, 6, 1536347485, NULL, 'o', 1536347485, NULL),
(175, 6, 1536340045, NULL, 'o', 1536340045, NULL),
(174, 6, 1535729373, NULL, 'o', 1535729373, NULL),
(173, 6, 1535728830, NULL, 'o', 1535728830, NULL),
(172, 6, 1535474782, NULL, 'o', 1535474782, NULL),
(171, 6, 1535007549, NULL, 'o', 1535007549, NULL),
(170, 6, 1534947249, NULL, 'o', 1534947249, NULL),
(169, 6, 1534527270, NULL, 'o', 1534527270, NULL),
(105, 3, 1513441762, 'tkehler', 'o', 1513441762, 'tkehler'),
(106, 2, 1513441796, 'tkehler', 'o', 1547369478, 'tkehler'),
(107, 3, 1513442013, 'tkehler', 'o', 1513442013, 'tkehler'),
(108, 2, 1513442053, 'tkehler', 'o', 1547369451, 'tkehler'),
(109, 3, 1513620137, 'tkehler', 'o', 1513620137, 'tkehler'),
(110, 2, 1513620224, 'tkehler', 'o', 1547370446, 'hfranzen'),
(111, 3, 1513620334, 'tkehler', 'o', 1513620334, 'tkehler'),
(112, 2, 1513620411, 'tkehler', 'o', 1547370455, 'hfranzen'),
(113, 3, 1513620778, 'tkehler', 'o', 1530102368, 'tkehler'),
(114, 2, 1513620850, 'tkehler', 'o', 1550492784, 'kkotenko'),
(168, 6, 1534494963, NULL, 'o', 1534494963, NULL),
(167, 6, 1534433568, NULL, 'o', 1534433568, NULL),
(166, 6, 1533660198, NULL, 'o', 1533660198, NULL),
(165, 6, 1533652031, 'tkehler', 'o', 1533652031, 'tkehler'),
(119, 3, 1519664713, 'tkehler', 'o', 1519664713, 'tkehler'),
(120, 3, 1519664811, 'tkehler', 'o', 1519664811, 'tkehler'),
(121, 3, 1519664835, 'tkehler', 'o', 1519664835, 'tkehler'),
(122, 3, 1519664867, 'tkehler', 'o', 1519664867, 'tkehler'),
(123, 3, 1519664897, 'tkehler', 'o', 1519665474, 'tkehler'),
(124, 2, 1519664943, 'tkehler', 'o', 1547369187, 'tkehler'),
(125, 2, 1519664985, 'tkehler', 'o', 1546934821, 'hfranzen'),
(126, 2, 1519665005, 'tkehler', 'o', 1546934861, 'hfranzen'),
(127, 2, 1519665059, 'tkehler', 'o', 1547370146, 'lbadaru'),
(128, 2, 1519665080, 'tkehler', 'o', 1547369195, 'tkehler'),
(129, 3, 1526222684, 'paradyx', 'o', 1526222814, 'paradyx'),
(130, 3, 1526222802, 'paradyx', 'o', 1526222802, 'paradyx'),
(131, 2, 1526223191, 'paradyx', 'o', 1547370384, 'hfranzen'),
(132, 2, 1526224301, 'paradyx', 'o', 1547370393, 'hfranzen'),
(133, 3, 1526224412, 'paradyx', 'o', 1526224412, 'paradyx'),
(134, 3, 1526224439, 'paradyx', 'o', 1526224439, 'paradyx'),
(135, 3, 1526224476, 'paradyx', 'o', 1526224476, 'paradyx'),
(136, 3, 1526224493, 'paradyx', 'o', 1526224493, 'paradyx'),
(137, 2, 1526224643, 'paradyx', 'o', 1547370271, 'lwontroba'),
(138, 2, 1526224689, 'paradyx', 'o', 1547370080, 'tkehler'),
(139, 2, 1526224733, 'paradyx', 'o', 1547370373, 'hfranzen'),
(140, 2, 1526224766, 'paradyx', 'o', 1547370090, 'tkehler'),
(141, 3, 1526732011, 'tkehler', 'o', 1526732011, 'tkehler'),
(142, 3, 1526732031, 'tkehler', 'o', 1526732031, 'tkehler'),
(143, 3, 1526732066, 'tkehler', 'o', 1526732066, 'tkehler'),
(144, 2, 1526732105, 'tkehler', 'o', 1547370419, 'hfranzen'),
(145, 2, 1526732138, 'tkehler', 'o', 1547370427, 'hfranzen'),
(146, 2, 1526732156, 'tkehler', 'o', 1547370401, 'hfranzen'),
(147, 3, 1530102791, 'tkehler', 'o', 1530102791, 'tkehler'),
(148, 2, 1530102894, 'tkehler', 'o', 1547369391, 'tkehler'),
(149, 3, 1530102932, 'tkehler', 'o', 1530102932, 'tkehler'),
(150, 3, 1530102946, 'tkehler', 'o', 1530102946, 'tkehler'),
(151, 2, 1530103041, 'tkehler', 'o', 1547369214, 'tkehler'),
(152, 2, 1530103066, 'tkehler', 'o', 1547369222, 'tkehler'),
(153, 3, 1530103088, 'tkehler', 'o', 1530103088, 'tkehler'),
(154, 2, 1530103136, 'tkehler', 'o', 1547369231, 'tkehler'),
(155, 3, 1530103171, 'tkehler', 'o', 1530103171, 'tkehler'),
(156, 2, 1530103244, 'tkehler', 'o', 1547369275, 'tkehler'),
(157, 3, 1530103282, 'tkehler', 'o', 1530103282, 'tkehler'),
(158, 3, 1530103299, 'tkehler', 'o', 1530103299, 'tkehler'),
(159, 2, 1530103400, 'tkehler', 'o', 1547369249, 'tkehler'),
(160, 2, 1530103441, 'tkehler', 'o', 1547369259, 'tkehler'),
(161, 3, 1530103473, 'tkehler', 'o', 1530103587, 'tkehler'),
(162, 2, 1530103610, 'tkehler', 'o', 1547369267, 'tkehler'),
(163, 3, 1533543169, 'tkehler', 'o', 1533543169, 'tkehler'),
(164, 2, 1533543229, 'tkehler', 'o', 1547370589, 'cbeck'),
(183, 6, 1541358281, NULL, 'o', 1541358281, NULL),
(184, 6, 1541617335, NULL, 'o', 1541617335, NULL),
(185, 3, 1542703481, 'tkehler', 'o', 1542703481, 'tkehler'),
(186, 2, 1542703534, 'tkehler', 'o', 1547369293, 'tkehler'),
(187, 6, 1542737335, NULL, 'o', 1542737335, NULL),
(188, 6, 1543346731, NULL, 'o', 1543346731, NULL),
(189, 6, 1543627029, NULL, 'o', 1543627029, NULL),
(190, 6, 1544522837, '', 'o', 1544522837, ''),
(191, 6, 1544957061, '', 'o', 1544957061, ''),
(192, 3, 1547371048, 'tkehler', 'o', 1547371048, 'tkehler'),
(193, 3, 1547371108, 'tkehler', 'o', 1547371999, 'tkehler'),
(194, 3, 1547371128, 'tkehler', 'o', 1547371128, 'tkehler'),
(195, 3, 1547371154, 'tkehler', 'o', 1547371154, 'tkehler'),
(196, 3, 1547371182, 'tkehler', 'o', 1547371546, 'tkehler'),
(197, 2, 1547371250, 'tkehler', 'o', 1547371250, 'tkehler'),
(198, 2, 1547371307, 'tkehler', 'o', 1547371307, 'tkehler'),
(199, 2, 1547371361, 'tkehler', 'o', 1547371361, 'tkehler'),
(200, 2, 1547371387, 'tkehler', 'o', 1547371387, 'tkehler'),
(201, 2, 1547371408, 'tkehler', 'o', 1547371408, 'tkehler');

-- --------------------------------------------------------

--
-- Table structure for table `tiki_tracker_item_attachments`
--

CREATE TABLE `tiki_tracker_item_attachments` (
  `attId` int(12) NOT NULL,
  `itemId` int(12) NOT NULL DEFAULT '0',
  `filename` varchar(80) COLLATE utf8_unicode_ci DEFAULT NULL,
  `filetype` varchar(80) COLLATE utf8_unicode_ci DEFAULT NULL,
  `filesize` int(14) DEFAULT NULL,
  `user` varchar(200) COLLATE utf8_unicode_ci DEFAULT NULL,
  `data` longblob,
  `path` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  `hits` int(10) DEFAULT NULL,
  `created` int(14) DEFAULT NULL,
  `comment` varchar(250) COLLATE utf8_unicode_ci DEFAULT NULL,
  `longdesc` blob,
  `version` varchar(40) COLLATE utf8_unicode_ci DEFAULT NULL
) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Indexes for dumped tables
--

--
-- Indexes for table `tiki_trackers`
--
ALTER TABLE `tiki_trackers`
  ADD PRIMARY KEY (`trackerId`);

--
-- Indexes for table `tiki_tracker_fields`
--
ALTER TABLE `tiki_tracker_fields`
  ADD PRIMARY KEY (`fieldId`),
  ADD UNIQUE KEY `permName` (`permName`,`trackerId`),
  ADD KEY `trackerId` (`trackerId`);

--
-- Indexes for table `tiki_tracker_items`
--
ALTER TABLE `tiki_tracker_items`
  ADD PRIMARY KEY (`itemId`),
  ADD KEY `trackerId` (`trackerId`);

--
-- Indexes for table `tiki_tracker_item_attachments`
--
ALTER TABLE `tiki_tracker_item_attachments`
  ADD PRIMARY KEY (`attId`),
  ADD KEY `itemId` (`itemId`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `tiki_trackers`
--
ALTER TABLE `tiki_trackers`
  MODIFY `trackerId` int(12) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=7;
--
-- AUTO_INCREMENT for table `tiki_tracker_fields`
--
ALTER TABLE `tiki_tracker_fields`
  MODIFY `fieldId` int(12) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=35;
--
-- AUTO_INCREMENT for table `tiki_tracker_items`
--
ALTER TABLE `tiki_tracker_items`
  MODIFY `itemId` int(12) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=202;
--
-- AUTO_INCREMENT for table `tiki_tracker_item_attachments`
--
ALTER TABLE `tiki_tracker_item_attachments`
  MODIFY `attId` int(12) NOT NULL AUTO_INCREMENT;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
