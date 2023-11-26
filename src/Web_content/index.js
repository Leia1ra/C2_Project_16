let movieNameRef = document.getElementById("OMDB_movie-name");
let searchBtn = document.getElementById("OMDB_search-btn");
let result = document.getElementById("OMDB_result");

//function to fetch data from api

let getMovie = () => {
    let movieName = movieNameRef.value;
    let url = `http://www.omdbapi.com/?t=${movieName}&apikey=${key}`;
    //if input field is empty

    if (movieName.length <= 0) {
        result.innerHTML = `<h3 class="msg">Please enter a movie name </h3>`;
    }

    //if input isn't empty
    else {
        fetch(url).then((resp) => resp.json()).then((data) => {
            //if movie exist in database
            if (data.Response == "True") {
                let icons = ''; // 아이콘 태그들을 저장할 변수
                // AJAX 요청
                fetch('/search', { 
                    method : 'post',
                    headers: {'Content-Type' : 'application/json'},
                    body : JSON.stringify(movieName)})
                .then(function(response) {
                    if(response.ok) { return    response.json();    } 
                    else            { throw     new Error('error'); }
                })
                .then(searched_MV => {
                    searched_MV.forEach(movie => {
                        const platform = movie.ott.split(","); // ,로 구분된 값을 배열로 분할
            
                        const ottServices = [
                            { name: '넷플릭스', activeIcon: 'icons/넷플릭스.png', inactiveIcon: 'icons/넷플릭스 비활성화.png' },
                            { name: '왓챠', activeIcon: 'icons/왓챠.png', inactiveIcon: 'icons/왓챠 비활성화.png' },
                            { name: '웨이브', activeIcon: 'icons/웨이브.png', inactiveIcon: 'icons/웨이브 비활성화.png' },
                            { name: '쿠팡', activeIcon: 'icons/쿠팡.png', inactiveIcon: 'icons/쿠팡 비활성화.png' },
                            { name: '티빙', activeIcon: 'icons/티빙.png', inactiveIcon: 'icons/티빙 비활성화.png' },
                            { name: '디즈니플러스', activeIcon: 'icons/디즈니플러스.png', inactiveIcon: 'icons/디즈니플러스 비활성화.png' }
                        ];
            
                        ottServices.forEach(ottService => {
                            const isActive = platform.includes(ottService.name);
                            const iconPath = isActive ? ottService.activeIcon : ottService.inactiveIcon;
                            icons += `<img src="${iconPath}" alt="${ottService.name}">`;
                        });
                    });
                    result.innerHTML = `
                    <div class="OMDB_info">
                        <img src=${data.Poster} class="OMDB_poster">
                            <div class = "OMDB_more-info">
                                <h2>${data.Title}</h2>
                                <div class="OMDB_rating">
                                    <img src="star-icon.svg">
                                    <h4>${data.imdbRating}</h4>
                                    <div class="OMDB_details">
                                    <span>${data.Rated}</span>
                                    <span>${data.Year}</span>
                                    <span>${data.Runtime}</span>
                                </div>
                                </div>
                                <div class="searched_ott">
                                    ${icons}
                                </div>
                                <div class="OMDB_genre">
                                    <div>${data.Genre.split(",").join("</div><div>")}</div>
                                </div>
                                
                                <h3>Plot:</h3>
                                <p>${data.Plot}</p>
                                
                                <h3>Cast:</h3>
                                <p>${data.Actors}</p>
                                
                            </div>
                        </div>
                    </div>`;
                })
            }

            //if movie doesn't exist in database
            else {
                result.innerHTML = `<h3 class="msg">${data.Error}</h3>`;
            }
        })
        //if error occurs
        .catch(() => {
            result.innerHTML = `<h3 class="msg">Error Occured</h3>`;
        });
    }
};


searchBtn.addEventListener("click", getMovie);
window.addEventListener("load", getMovie);

// var xhr = new 
                // XMLHttpRequest();
                // xhr.open('POST', '/search', true);
                // xhr.setRequestHeader('Content-Type', 'application/json');
                // xhr.onreadystatechange = function() {
                //     if (xhr.readyState === XMLHttpRequest.DONE) {
                //     if (xhr.status === 200) {
                //         // 서버에서 반환한 결과를 출력
                //         var response = JSON.parse(xhr.responseText);
                //         console.log(response.result);
                //     } else {
                //         console.error('요청에 실패했습니다.');
                //     }
                //     }
                // };
                // xhr.send(JSON.stringify(movieName));