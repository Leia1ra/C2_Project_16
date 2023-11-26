<?php
include "db.php";

$name = $_POST[ 'name' ];
$result = mysqli_query($conn, "SELECT * FROM ott_table WHERE 제목 = '$name';");
$row = mysqli_fetch_array($result); 
?>

<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>무비서</title>
  </head>
  <body>
  <h1>영화 정보</h1>
  <input type="hidden" name="name" value="<?php echo $row[ 'name' ]; ?>">

    <p>제목: <?php echo $row['제목']; ?></p>
    <p>장르: <?php echo $row['장르']; ?></p>
    <p>구분: <?php echo $row['구분']; ?></p> 
    <p>개봉연도: <?php echo $row['개봉연도']; ?></p>
    <p>평점: <?php echo $row['평점']; ?></p>
    <p>나이제한: <?php echo $row['나이제한']; ?></p>
    <p>위치: <?php echo $row['위치']; ?></p>
  </body>
</html>
