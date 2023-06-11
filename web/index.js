document.addEventListener("DOMContentLoaded", function () {
    const button = document.getElementById("submit-btn");
    const radio = document.getElementById("query-type");
    const textInput = document.getElementById("text-input")

    const radioInputs = document.querySelectorAll('input[name="query-type"]');

    radioInputs.forEach((radioInput) => {
        radioInput.addEventListener("click", function () {
            radio.value = `Search ${radioInput.value}`;
        })
    });

    const url = 'http://localhost:8080/artist';

    button.addEventListener("click", function (event) {

        event.preventDefault();

        console.log(textInput.value)

        const fullURL = url + "?artist_name=" + textInput.value
        fetch(fullURL)
            .then(response => response.json())
            .then(data => {
                // Process the data returned from the API
                console.log(data);
                // Get the container element
                const container = document.getElementById('artists-container');

                // Iterate over each artist object
                data.artists.forEach(artist => {
                    console.log(artist)
                    // Create elements to display artist information
                    const artistElement = document.createElement('div');
                    const nameElement = document.createElement('h2');
                    const followersElement = document.createElement('p');
                    const imageElement = document.createElement('img');

                    // Set the content of the elements
                    nameElement.textContent = artist.name;
                    followersElement.textContent = `followers: ${artist.followers.total}`;

                    const imageUrl = artist.images[1] ? artist.images[1].url : 'https://www.freepnglogos.com/images/spotify-logo-png-7053.html';
                    imageElement.setAttribute('src', imageUrl)

                    // Append the elements to the artist container
                    artistElement.appendChild(nameElement);
                    artistElement.appendChild(followersElement);
                    artistElement.appendChild(imageElement);
                    container.appendChild(artistElement);
                });
            })
            .catch(error => {
                // Handle any errors that occurred during the request
                console.log("failed here")
                console.error(error);
            });
    });
});