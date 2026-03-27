(function() {
    function getLang() {
        return localStorage.getItem('mdbook-lang') || 'bilingual';
    }

    function setLang(lang) {
        localStorage.setItem('mdbook-lang', lang);
        document.body.classList.remove('lang-en', 'lang-zh', 'lang-bilingual');
        if (lang !== 'bilingual') {
            document.body.classList.add('lang-' + lang);
        }
        updateButtonStatus(lang);
    }

    function updateButtonStatus(lang) {
        const btnEN = document.getElementById('btn-en');
        const btnZH = document.getElementById('btn-zh');
        const btnBI = document.getElementById('btn-bi');

        if (btnEN) btnEN.classList.toggle('active', lang === 'en');
        if (btnZH) btnZH.classList.toggle('active', lang === 'zh');
        if (btnBI) btnBI.classList.toggle('active', lang === 'bilingual');
    }

    function injectToggle() {
        const topBar = document.querySelector('.left-buttons');
        if (!topBar) return;

        const toggleDiv = document.createElement('div');
        toggleDiv.id = 'lang-toggle';
        toggleDiv.innerHTML = `
            <span id="btn-bi" class="lang-btn">A/中</span>
            <span id="btn-en" class="lang-btn">EN</span>
            <span id="btn-zh" class="lang-btn">中</span>
        `;

        topBar.appendChild(toggleDiv);

        toggleDiv.querySelector('#btn-bi').onclick = () => setLang('bilingual');
        toggleDiv.querySelector('#btn-en').onclick = () => setLang('en');
        toggleDiv.querySelector('#btn-zh').onclick = () => setLang('zh');

        setLang(getLang());
    }

    // Wait for DOM to be ready
    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", injectToggle);
    } else {
        injectToggle();
    }
})();
