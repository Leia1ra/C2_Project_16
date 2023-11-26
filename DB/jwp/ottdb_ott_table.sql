-- MySQL dump 10.13  Distrib 8.0.31, for Win64 (x86_64)
--
-- Host: localhost    Database: ottdb
-- ------------------------------------------------------
-- Server version	8.0.31

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `ott_table`
--

DROP TABLE IF EXISTS `ott_table`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `ott_table` (
  `제목` varchar(20) DEFAULT NULL,
  `장르` varchar(10) DEFAULT NULL,
  `구분` varchar(10) DEFAULT NULL,
  `개봉연도` smallint DEFAULT NULL,
  `평점` float DEFAULT NULL,
  `나이제한` varchar(10) DEFAULT NULL,
  `위치` varchar(5) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ott_table`
--

LOCK TABLES `ott_table` WRITE;
/*!40000 ALTER TABLE `ott_table` DISABLE KEYS */;
INSERT INTO `ott_table` VALUES ('SNL코리아','코미디','예능',2022,4.4,'19+','쿠팡'),('낭만닥터김사부','의학','드라마',2016,1.4,'15','쿠팡'),('엑시트','액션,코미디','영화',2019,4.6,'12세미만','쿠팡'),('인터스텔라','S.F','영화',2014,4.7,'12세미만','쿠팡'),('트와일라잇','판타지','영화',2008,4.6,'12세미만','쿠팡'),('뉴문','판타지','영화',2009,4.6,'12세미만','쿠팡'),('이클립스','판타지','영화',2010,4.5,'12세미만','쿠팡'),('비상선언','드라마','영화',2022,4,'12세미만','쿠팡'),('나는솔로','연예','예능',2021,4.3,'15세미만','쿠팡'),('모범택시','액션,코미디','드라마',2023,4.5,'15세미만','쿠팡'),('펜트하우스','범죄,스릴러','드라마',2020,4.2,'19+','쿠팡'),('태양의 후예','액션,로맨스','드라마',2016,4.5,'15세미만','쿠팡'),('오은영의비밀상담','리얼리티,토크쇼','예능',2021,4.3,'15세미만','쿠팡'),('놀면뭐하니?','리얼리티,토크쇼','예능',2019,4.5,'15세미만','쿠팡'),('나혼자산다','리얼리티,토크쇼','예능',2013,4.4,'15세미만','쿠팡'),('너의목소리가들려','법정','드라마',2013,4.5,'15세미만','쿠팡'),('걸캅스','범죄,스릴러','영화',2019,4.2,'15세미만','쿠팡'),('검사외전','범죄,코미디','영화',2016,4.5,'15세미만','쿠팡'),('화이트칙스','범죄,코미디','영화',2004,4.6,'15세미만','쿠팡'),('부산촌놈in시드니','리얼리티','예능',2023,4.5,'15세미만','티빙'),('시그널','범죄,스릴러','드라마',2016,4.5,'15세미만','티빙'),('사랑의불시착','연예','드라마',2019,4.5,'15세미만','티빙'),('도깨비','판타지','드라마',2016,4.6,'15세미만','티빙'),('빈센조','스릴러','드라마',2021,4.5,'15세미만','티빙'),('써니','드라마','영화',2011,4.6,'15세미만','티빙'),('수상한그녀','드라마','영화',2014,4.6,'15세미만','티빙'),('베놈','액션','영화',2018,4.5,'15세미만','티빙'),('기생충','드라마','영화',2019,4.7,'15세미만','티빙'),('윤식당','리얼리티','예능',2017,4.5,'15세미만','티빙'),('종이달','범죄,스릴러','드라마',2023,4.4,'19+','티빙'),('응답하라1988','드라마','드라마',2015,4.6,'15세미만','티빙'),('이태원클라쓰','드라마','드라마',2020,4.7,'15세미만','티빙'),('김비서가왜그럴까','멜로','드라마',2018,4.5,'15세미만','티빙'),('탑건매버릭','액션','영화',2022,4.7,'12세미만','티빙'),('나는솔로','연예','예능',2022,4.5,'15세미만','티빙'),('환승연애','리얼리티','예능',2022,4.5,'15세미만','티빙'),('여고추리반','스릴러','예능',2021,4.7,'15세미만','티빙'),('강철부대','리얼리티,토크쇼','예능',2022,4.3,'15세미만','쿠팡'),('방과후전쟁활동','스릴러','드라마',2023,4.2,'19+','티빙'),('금쪽같은내새끼','리얼리티','예능',2020,4.1,'12세미만','티빙');
/*!40000 ALTER TABLE `ott_table` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-05-16 22:10:28
