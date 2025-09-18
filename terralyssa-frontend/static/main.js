document.getElementById("test_btn").addEventListener("click", async () => {
    const res = await fetch('/clicked', { method: 'POST' });
    const data = await res.json();
    document.getElementById("test_output").innerText = data.message;
});
