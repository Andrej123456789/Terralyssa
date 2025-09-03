document.getElementById("test").addEventListener("click", async () => {
    const res = await fetch('/clicked', { method: 'POST' });
    const data = await res.json();
    document.getElementById("result").innerText = data.message;
});
