document.addEventListener("DOMContentLoaded", function (event) {
    const button = document.getElementById("submit-btn");
    const text = document.getElementById("text-input");

    const radioInputs = document.querySelectorAll('input[name="query-type"]');

    radioInputs.forEach((radioInput) => {
        radioInput.addEventListener("click", function () {
            text.innerText = `Search ${radioInput.value}`;
        })
    });

    const url = 'http://localhost:8080';

    button.addEventListener("click", function () {
        fetch(url)
            .then(response => response.json())
            .then(data => {
                // Process the data returned from the API
                console.log(data);
            })
            .catch(error => {
                // Handle any errors that occurred during the request
                console.error(error);
            });
    });
});