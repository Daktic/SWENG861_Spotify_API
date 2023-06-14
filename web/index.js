function formatTime(milliseconds) {
    // Convert milliseconds to seconds
    const totalSeconds = Math.floor(milliseconds / 1000);

    // Calculate minutes and seconds
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;

    // Return the formatted time as a string
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}


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

    button.disabled = textInput.value.trim() === '';

    textInput.addEventListener('input', function () {
        button.disabled = textInput.value.trim() === '';
    });

    button.addEventListener("click", function () {

        document.getElementById('artists-container').innerHTML = ""; // Clears screen on new search
        document.getElementById('song-container').innerHTML = "";
        document.getElementById('error-container').innerHTML = "";


        const fullURL = url + `?${query_type}_name=` + encodeURIComponent(textInput.value);
        console.log(fullURL)
        fetch(fullURL)
            .then(response => response.json())
            .then(data => {
                // Get the container element


                console.log(data);

                if (data.artists && data.artists.length > 0) {
                    const container = document.getElementById('artists-container');
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

                        const external_link = document.createElement('a');
                        external_link.setAttribute('href', artist.external_urls.spotify);


                        // Append the elements to the artist container
                        artistStats.appendChild(followersElement);
                        artistStats.appendChild(genresElement);
                        external_link.appendChild(nameElement);
                        artistItems.appendChild(imageElement);
                        artistItems.appendChild(artistStats);
                        artistElement.appendChild(external_link);
                        artistElement.appendChild(artistItems);


                        container.appendChild(artistElement);
                    });
                } else if (data.songs && data.songs.length > 0) {
                    const container = document.getElementById('song-container');
                    // Iterate over each artist object
                    data.songs.forEach(song => {
                        // Create elements to display song information
                        const songElement = document.createElement('div');
                        const songStats = document.createElement('div');
                        const songItems = document.createElement('div');
                        const nameElement = document.createElement('h2');
                        const artistNameElement = document.createElement('h3');
                        const albumNameElement = document.createElement('p');
                        const songLengthElement = document.createElement('p');
                        const songExplicitElement = document.createElement('p');
                        const genresElement = document.createElement('p');
                        const imageElement = document.createElement('img');

                        // Set the content of the elements
                        nameElement.textContent = song.name;
                        nameElement.setAttribute('class', 'song-name');
                        artistNameElement.textContent = "By: " + song.artists[0].name;
                        artistNameElement.setAttribute('class', 'artist-name');

                        songLengthElement.textContent = `Length (Minutes:Seconds): ${formatTime(song.duration_ms)}`
                        songLengthElement.setAttribute('class', 'song-length');

                        albumNameElement.textContent = `Album: ${song.album.name}`;
                        albumNameElement.setAttribute('class', 'song-album-name');

                        const imageUrl = song.album.images[1] ? song.album.images[1].url : 'https://www.freepnglogos.com/images/spotify-logo-png-7053.html';
                        imageElement.setAttribute('src', imageUrl)
                        imageElement.setAttribute('class', 'song-image')

                        songElement.setAttribute('class', 'song-card')
                        songItems.setAttribute('class', 'song-items')
                        songStats.setAttribute('class', 'song-stats')
                        song.explicit ? songExplicitElement.textContent = `Explicit` : '';
                        songExplicitElement.setAttribute('class', 'explicit')

                        const external_link = document.createElement('a');
                        external_link.setAttribute('href', song.external_urls.spotify);
                        external_link.setAttribute('target', "_");

                        // Append the elements to the song container
                        songStats.appendChild(songLengthElement);
                        songStats.appendChild(genresElement);
                        songStats.appendChild(albumNameElement);
                        songStats.appendChild(songExplicitElement);
                        external_link.appendChild(nameElement);
                        songElement.appendChild(external_link);
                        songElement.appendChild(artistNameElement);
                        songItems.appendChild(imageElement);
                        songItems.appendChild(songStats);
                        songElement.appendChild(songItems);


                        container.appendChild(songElement);
                    })
                } else {
                    const container = document.getElementById('error-container');

                    const errTitleElement = document.createElement('h2');
                    const errSubTitleElement = document.createElement('h3');

                    errTitleElement.innerText = "Nothing Found!"
                    errSubTitleElement.innerText = "Try another search"

                    container.appendChild(errTitleElement);
                    container.appendChild(errSubTitleElement);
                }
            })
            .catch(error => {
                // Handle any errors that occurred during the request
                const container = document.getElementById('error-container');
                const errorCard = document.createElement('div');
                const errTitleElement = document.createElement('h2');
                const errSubTitleElement = document.createElement('h3');
                const errMessageElement = document.createElement('p');

                errTitleElement.innerText = "Error!"
                errSubTitleElement.innerText = "Something went wrong:"
                errMessageElement.innerText = error.toString();//not currently working
                console.log(error.message);

                errorCard.appendChild(errTitleElement);
                errorCard.appendChild(errSubTitleElement);
                errorCard.appendChild(errMessageElement);
                container.appendChild(errorCard);
            });
    });
});