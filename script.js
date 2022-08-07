export function get() {
    fetch('https://api.dictionaryapi.dev/api/v2/entries/en/cybersecurity')
        .then(response => response.json())
        .then(data => {
            document.getElementById("result").innerHTML="Cybersecurity: "+(data[0].meanings[0].definitions[0].definition)
        })
        .catch(error => console.error(error))
}