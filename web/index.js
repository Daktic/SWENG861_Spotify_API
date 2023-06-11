document.addEventListener("DOMContentLoaded", function () {
    const button = document.getElementById("submit-btn");
    const radio = document.getElementById("query-type");
    const textInput = document.getElementById("text-input")

    const radioInputs = document.querySelectorAll('input[name="query-type"]');

    radioInputs.forEach((radioInput) => {
        radioInput.addEventListener("click", function () {
            radio.innerText = `Search ${radioInput.value}`;
        })
    });

    const url = 'http://localhost:8080/artist';

    button.addEventListener("click", function () {

        document.getElementById('artists-container').innerHTML = ""; // Clears screen on new search

        const fullURL = url + "?artist_name=" + textInput.value
        fetch(fullURL)
            .then(response => response.json())
            .then(data => {
                // Get the container element
                const container = document.getElementById('artists-container');

                // Iterate over each artist object
                data.artists.forEach(artist => {
                    // Create elements to display artist information
                    const artistElement = document.createElement('div');
                    const artistStats = document.createElement('div');
                    const artistItems = document.createElement('div');
                    const nameElement = document.createElement('h2');
                    const followersElement = document.createElement('p');
                    const imageElement = document.createElement('img');

                    // Set the content of the elements
                    nameElement.textContent = artist.name;
                    nameElement.setAttribute('class', 'artist-name');
                    followersElement.textContent = `followers: ${artist.followers.total}`;
                    followersElement.setAttribute('class', 'artist-followers');

                    const imageUrl = artist.images[1] ? artist.images[1].url : 'https://www.freepnglogos.com/images/spotify-logo-png-7053.html';
                    imageElement.setAttribute('src', imageUrl)
                    imageElement.setAttribute('class', 'artist-image')

                    artistElement.setAttribute('class', 'artist-card')
                    artistItems.setAttribute('class', 'artist-items')
                    artistStats.setAttribute('class', 'artist-stats')

                    // Append the elements to the artist container
                    artistStats.appendChild(followersElement);
                    artistItems.appendChild(nameElement);
                    artistItems.appendChild(imageElement);
                    artistElement.appendChild(artistItems);
                    artistElement.appendChild(artistStats);

                    container.appendChild(artistElement);
                });
            })
            .catch(error => {
                // Handle any errors that occurred during the request
                console.error(error);
            });
    });
});