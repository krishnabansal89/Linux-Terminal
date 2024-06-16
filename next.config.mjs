/** @type {import('next').NextConfig} */
const nextConfig = {
    output:"export",
    reactStrictMode: true,
    // Note: This feature is required to use NextJS Image in SSG mode.
    // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
    images: {
      unoptimized: true,
    },
    distDir: 'out', // This is the default value, but it's good to be explicit
  };
  

export default nextConfig;
