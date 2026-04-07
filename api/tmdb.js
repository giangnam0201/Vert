

export default async function handler(req, res) {
    if (req.method !== 'GET') {
        return res.status(405).json({ error: 'Phương thức không được phép' });
    }

    const apiKey = process.env.TMDB_API_KEY;
    if (!apiKey) {
        return res.status(500).json({ error: 'TMDB_API_KEY chưa được cấu hình' });
    }

    const { ep, ...params } = req.query;
    if (!ep) {
        return res.status(400).json({ error: 'Thiếu tham số truy vấn "ep"' });
    }

    const ALLOWED = [
        '/trending/', '/movie/', '/tv/', '/search/', '/genre/', '/discover/'
    ];
    if (!ALLOWED.some(prefix => ep.startsWith(prefix))) {
        return res.status(403).json({ error: 'Endpoint không được phép' });
    }

    try {
        const sep = ep.includes('?') ? '&' : '?';
        let url = `https://api.themoviedb.org/3${ep}${sep}api_key=${apiKey}&language=vi-VN`;

        Object.entries(params).forEach(([k, v]) => {
            url += `&${k}=${encodeURIComponent(v)}`;
        });

        const tmdbRes = await fetch(url);
        const data = await tmdbRes.json();

        res.setHeader('Cache-Control', 's-maxage=600, stale-while-revalidate=3600');
        return res.status(tmdbRes.status).json(data);
    } catch (err) {
        console.error('TMDB proxy error:', err);
        return res.status(502).json({ error: 'Không thể lấy dữ liệu từ TMDB' });
    }
}
