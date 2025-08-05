export function load({ cookies }) {
    const visited = cookies.get('visited');

    cookies.set('visited', 'true', { path: '/sveltekit/basic/header' });

    return {
        visited: visited === 'true'
    };
}