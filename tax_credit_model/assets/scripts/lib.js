function parseScriptParameters() {
    const src = document.currentScript.src;
    let url = new URL(src);
    url.searchParams.get('state');
    const query = src.replace(/^[^\?]+\??/, '');

    if (!query) {
        return new Object();
    }

    const queryParameters = new Object();
    const pairs = query.split(/[;&]/);

    for (let i = 0; i < pairs.length; i++) {
        const parameter = pairs[i].split('=');

        if (!parameter || parameter.length != 2) {
            continue;
        };

        const key = decodeURIComponent(parameter[0]);
        let val = decodeURIComponent(parameter[1]);
        val = val.replace(/\+/g, ' ');
        queryParameters[key] = val;
    }

    return queryParameters;
}
