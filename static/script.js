const unsafeBtn = document.getElementById('unsafe-btn');
const safeBtn = document.getElementById('safe-btn');
const output = document.getElementById('output');

async function fetchData(endpoint) {
    const btn = endpoint === 'unsafe' ? unsafeBtn : safeBtn;
    btn.classList.add('btn-loading');
    output.textContent = 'Loading...';

    try {
        const response = await fetch(`http://localhost:3000/api/${endpoint}`);
        if (!response.ok) throw new Error('Network response was not ok');
        const data = await response.json();
        output.textContent = data.join('\n');
    } catch (error) {
        output.textContent = `Error: ${error.message}`;
        output.classList.add('text-red-500');
    } finally {
        btn.classList.remove('btn-loading');
        setTimeout(() => output.classList.remove('text-red-500'), 3000);
    }
}

unsafeBtn.addEventListener('click', () => fetchData('unsafe'));
safeBtn.addEventListener('click', () => fetchData('safe')); 