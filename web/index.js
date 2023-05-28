document.addEventListener("DOMContentLoaded", function (event) {
    const button = document.getElementById("submit-btn");
    const text = document.getElementById("text-input");

    const radioInputs = document.querySelectorAll('input[name="query-type"]');

    radioInputs.forEach((radioInput) => {
        radioInput.addEventListener("click", function () {
            text.innerText = `Search ${radioInput.value}`;
        })
    });

    button.addEventListener("click", function () {

    });
});