document.addEventListener("DOMContentLoaded", function () {
    const button = document.getElementById("submit-btn");
    const radio = document.getElementById("query-type");
    const textInput = document.getElementById("text-input")

    const radioInputs = document.querySelectorAll('input[name="query-type"]');

    let query_type = 'artist';
    let url = `http://localhost:8080/${query_type}`;


    radioInputs.forEach((radioInput) => {
        radioInput.addEventListener("click", function () {
            query_type = radioInput.value.toLowerCase();
            url = `http://localhost:8080/${query_type}`; //IDK why i need to specify this here

            radio.innerText = `Search ${radioInput.value}`;
        })
    });


    button.addEventListener("click", function () {

        document.getElementById('artists-container').innerHTML = ""; // Clears screen on new search

        //const fullURL = url + "?artist_name=" + textInput.value;
        const fullURL = url + `?${query_type}_name=` + textInput.value;
        fetch(fullURL)
            .then(response => response.json())
            .then(data => {
                // Get the container element
                const container = document.getElementById('artists-container');

                console.log(data);

                // Iterate over each artist object
                data.artists.forEach(artist => {
                    // Create elements to display artist information
                    const artistElement = document.createElement('div');
                    const artistStats = document.createElement('div');
                    const artistItems = document.createElement('div');
                    const nameElement = document.createElement('h2');
                    const followersElement = document.createElement('p');
                    const genresElement = document.createElement('p');
                    const imageElement = document.createElement('img');

                    // Set the content of the elements
                    nameElement.textContent = artist.name;
                    nameElement.setAttribute('class', 'artist-name');
                    followersElement.setAttribute('class', 'artist-followers');
                    followersElement.textContent = `Followers: ${artist.followers.total}`;

                    genresElement.setAttribute('class', 'artist-genres');

                    let genres;
                    if (artist.genres.length < 1) {
                        genres = 'None'
                    } else {
                        genres = artist.genres.join(', ')
                    }
                    genresElement.textContent = `Genres: ${genres}`

                    const imageUrl = artist.images[1] ? artist.images[1].url : 'https://www.freepnglogos.com/images/spotify-logo-png-7053.html';
                    imageElement.setAttribute('src', imageUrl)
                    imageElement.setAttribute('class', 'artist-image')

                    artistElement.setAttribute('class', 'artist-card')
                    artistItems.setAttribute('class', 'artist-items')
                    artistStats.setAttribute('class', 'artist-stats')

                    // Append the elements to the artist container
                    artistStats.appendChild(followersElement);
                    artistStats.appendChild(genresElement);
                    artistElement.appendChild(nameElement);
                    artistItems.appendChild(imageElement);
                    artistItems.appendChild(artistStats);
                    artistElement.appendChild(artistItems);


                    container.appendChild(artistElement);
                });
            })
            .catch(error => {
                // Handle any errors that occurred during the request
                console.error(error);
            });
    });
});