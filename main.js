async function tts(text, lang, options = {}) {
    const { config, utils } = options;
    const { tauriFetch, CryptoJS } = utils;

    let language = {
        "zh_cn": "zh",
        "zh_tw": "zh",
        "en": "en",
        "ja": "jp",
        "ko": "kr",
        "fr": "fr",
        "es": "es",
        "ru": "ru",
        "de": "de",
        "it": "it",
        "tr": "tr",
        "pt_pt": "pt",
        "pt_br": "pt",
        "vi": "vi",
        "ms": "ms",
        "ar": "ar",
        "hi": "id",
    }[lang];

    if (!language) {
        throw "Language not Support!";
    }

    let speaker = config[`${lang}-speaker`];
    if (!speaker) {
        speaker = {
            "zh_cn": "zh_male_xiaoming",
            "zh_tw": "zh_male_xiaoming",
            "en": "en_male_adam",
            "ja": "jp_male_satoshi",
            "ko": "kr_male_gye",
            "fr": "fr_male_enzo",
            "es": "es_male_george",
            "ru": "tts.other.BV068_streaming",
            "de": "de_female_sophie",
            "it": "tts.other.BV087_streaming",
            "tr": "tts.other.BV083_streaming",
            "pt_pt": "pt_female_alice",
            "pt_br": "pt_female_alice",
            "vi": "tts.other.BV074_streaming",
            "ms": "tts.other.BV092_streaming",
            "ar": "tts.other.BV570_streaming",
            "hi": "id_female_noor",
        }[lang];
    }
    let headers = {
        "authority": "translate.volcengine.com",
        "origin": "chrome-extension://klgfhbdadaspgppeadghjjemk",
        "accept": "application/json, text/plain, */*",
        "sec-fetch-dest": "empty",
        "sec-fetch-mode": "cors",
        "sec-fetch-site": "none",
        "cookie": "hasUserBehavior=1",
        "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
    }
    const res = await tauriFetch(`https://translate.volcengine.com/crx/tts/v1/`, {
        method: "POST",
        headers: headers,
        body: {
            type: "Json",
            payload: {
                text,
                speaker,
                language
            }
        }
    });

    if (res.ok) {
        let result = res.data;
        if (result.audio && result.audio.data) {
            let base64 = result.audio.data;
            let data = CryptoJS.enc.Base64.parse(base64);
            let bytes = [];
            for (let i = 0; i < data.sigBytes; i++) {
                let byte = (data.words[i >>> 2] >>> (24 - (i % 4) * 8)) & 0xFF;
                bytes.push(byte);
            }
            return bytes;
        } else {
            throw JSON.stringify(result);
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}