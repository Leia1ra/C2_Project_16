let moviesList = document.getElementById('movies-list');

fetch('/movies', { method: 'POST' }).then(resp => resp.json())
    .then(movies => {
        let htmlContent = '';
        
        movies.forEach(movie => {
            const ottList = movie.ott.split(","); // ,로 구분된 값을 배열로 분할
            let ottIcons = ''; // 아이콘 태그들을 저장할 변수

            const ottServices = [
                { name: '넷플릭스', activeIcon: 'icons/넷플릭스.png', inactiveIcon: 'icons/넷플릭스 비활성화.png' },
                { name: '왓챠', activeIcon: 'icons/왓챠.png', inactiveIcon: 'icons/왓챠 비활성화.png' },
                { name: '웨이브', activeIcon: 'icons/웨이브.png', inactiveIcon: 'icons/웨이브 비활성화.png' },
                { name: '쿠팡', activeIcon: 'icons/쿠팡.png', inactiveIcon: 'icons/쿠팡 비활성화.png' },
                { name: '티빙', activeIcon: 'icons/티빙.png', inactiveIcon: 'icons/티빙 비활성화.png' },
                { name: '디즈니플러스', activeIcon: 'icons/디즈니플러스.png', inactiveIcon: 'icons/디즈니플러스 비활성화.png' }            
            ];

            ottServices.forEach(ottService => {
                const isActive = ottList.includes(ottService.name);
                const iconPath = isActive ? ottService.activeIcon : ottService.inactiveIcon;
                ottIcons += `<img src="${iconPath}" alt="${ottService.name}">`;
            });

            const movieItem = `
            <div class="poster_container">
                <div class="card">
                    <div class="face face1">
                        <div class="poster_content">
                            <img src="${movie.poster}" alt="Movie Image">
                        </div>
                    </div>

                    <div class="face face2">
                        <h5>
                            ${movie.ko}
                            <div class="eng">${movie.en}</div>
                        </h5>
                        <div class="rating">
                            <img src="star-icon.svg">
                            <h6>${movie.rate}</h6>
                        </div>
                        <div class="more">
                            <div>${movie.age}</div>
                            <div>${movie.release}</div>
                            <div>${movie.runtime}</div>
                        </div>
                        <div class="genre">
                            <div>${movie.genre.split(",").join("</div><div>")}</div>
                        </div>
                        <div class="ott">
                            ${ottIcons}
                        </div>
                    </div>
                </div>
            </div>`;
            htmlContent += movieItem;
        });
        // moviesList 요소의 HTML 내용을 변경
        moviesList.innerHTML = htmlContent;
    })
    .catch(error => console.error(error));

