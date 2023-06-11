document.addEventListener("DOMContentLoaded", function (event) {
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
            })
            .catch(error => {
                // Handle any errors that occurred during the request
                console.log("failed here")
                console.error(error);
            });
    });
});