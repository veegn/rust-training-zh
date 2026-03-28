(function () {
  const STORAGE_KEY = "rust-training-language";

  function normalizePath(path) {
    try {
      return new URL(path, window.location.origin).pathname;
    } catch (_) {
      return path;
    }
  }

  function swapLanguage(pathname, targetLang) {
    if (!pathname) return null;
    if (pathname.includes("/en/")) return pathname.replace("/en/", `/${targetLang}/`);
    if (pathname.includes("/zh/")) return pathname.replace("/zh/", `/${targetLang}/`);
    return null;
  }

  function collectSidebarPaths() {
    return Array.from(document.querySelectorAll(".sidebar a"))
      .map((anchor) => normalizePath(anchor.getAttribute("href") || ""))
      .filter(Boolean);
  }

  function firstSidebarPathFor(lang) {
    const anchors = Array.from(document.querySelectorAll(".sidebar a"));
    for (const anchor of anchors) {
      const path = normalizePath(anchor.getAttribute("href") || "");
      if (path.includes(`/${lang}/`)) {
        return path;
      }
    }
    return null;
  }

  function detectCurrentLang(pathname) {
    if (pathname.includes("/en/")) return "en";
    if (pathname.includes("/zh/")) return "zh";
    return null;
  }

  function buildToggle() {
    let nav = document.querySelector(".language-toggle");
    if (!nav) {
      const main = document.querySelector(".content main");
      if (!main) return;
      nav = document.createElement("nav");
      nav.className = "language-toggle";
      nav.setAttribute("aria-label", "Language switcher / 语言切换");
      main.prepend(nav);
    }
    if (!nav) return;

    const currentPath = window.location.pathname;
    const currentLang = detectCurrentLang(currentPath);
    const sidebarPaths = collectSidebarPaths();
    const preferredLang = localStorage.getItem(STORAGE_KEY);
    const fallbackLang = preferredLang === "zh" ? "zh" : "en";

    const targets = {
      en: swapLanguage(currentPath, "en") || firstSidebarPathFor("en"),
      zh: swapLanguage(currentPath, "zh") || firstSidebarPathFor("zh"),
    };

    if (!targets.en && !targets.zh) {
      return;
    }

    ["en", "zh"].forEach((lang) => {
      const anchor =
        nav.querySelector(`[data-lang-switch="${lang}"]`) || document.createElement("a");
      const href = targets[lang];
      const hasPage = href && sidebarPaths.includes(normalizePath(href));
      anchor.className = "language-toggle__button";
      anchor.textContent = lang === "en" ? "EN" : "中文";

      if ((currentLang || fallbackLang) === lang) {
        anchor.classList.add("is-active");
      } else {
        anchor.classList.remove("is-active");
      }

      if (hasPage) {
        anchor.href = href + window.location.search + window.location.hash;
        anchor.removeAttribute("aria-disabled");
        anchor.addEventListener("click", () => {
          localStorage.setItem(STORAGE_KEY, lang);
        });
      } else {
        anchor.href = "#";
        anchor.setAttribute("aria-disabled", "true");
      }

      if (!anchor.parentElement) {
        nav.appendChild(anchor);
      }
    });
  }

  document.addEventListener("DOMContentLoaded", buildToggle);
})();
