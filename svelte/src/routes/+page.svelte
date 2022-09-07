<script lang="ts">
    import BoxArrowUpRight from "../icons/box-arrow-up-right.svelte";
</script>

<nav class="bg-primary">
    <div class="container">
        <div class="row pb-4 gy-4">
            <div class="col-sm">
                <h2 class="my-4"><a href="/" class="text-white text-decoration-none">Mavinote</a></h2>
            </div>
            <div class="col-sm d-none d-md-flex justify-content-end align-items-center">
                <a class="btn btn-outline-light" href="/app">Start using Mavinote</a>
            </div>
        </div>
        <a rel="external" href="https://github.com/bwqr/mavinote" target="_blank">
            <svg class="octocat" viewBox="0 0 250 250">
                <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                <path class="octocat-arm" d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2"></path>
                <path class="octocat-body" d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z"></path>
            </svg>
        </a>
    </div>
</nav>
<section class="bg-primary">
    <div class="container">
        <div class="row gy-4">
            <div class="col-sm d-flex align-items-center">
                <div>
                    <h1 class="text-white">Open Source Note-Taking Application</h1>
                    <p class="text-white">Mavinote is a simple, cross-platform and open source note-taking application. It has Android, iOS and Web applications.</p>
                    <a class="d-block d-md-inline-block btn btn-outline-light" href="/app">Start Using Mavinote</a>
                </div>
            </div>
            <div class="col-sm">
                <img class="w-100" src="images/details.png" alt="software-app" />
            </div>
        </div>
    </div>
</section>

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1920 310" style="margin-top: -1px">
    <path
        fill="var(--bs-primary)"
        d="M0,283.054c22.75,12.98,53.1,15.2,70.635,14.808,92.115-2.077,238.3-79.9,354.895-79.938,59.97-.019,106.17,18.059,141.58,34,47.778,21.511,47.778,21.511,90,38.938,28.418,11.731,85.344,26.169,152.992,17.971,68.127-8.255,115.933-34.963,166.492-67.393,37.467-24.032,148.6-112.008,171.753-127.963,27.951-19.26,87.771-81.155,180.71-89.341,72.016-6.343,105.479,12.388,157.434,35.467,69.73,30.976,168.93,92.28,256.514,89.405,100.992-3.315,140.276-41.7,177-64.9V0.24H0V283.054Z"
    />
</svg>

<section class="container mt-5">
    <h1 class="text-center">Availability</h1>

    <div>
        <p>
            Mavinote has Android, iOS and Web applications.
            It is also available on
            <a rel="external nofollow" href="https://f-droid.org/packages/com.bwqr.mavinote/" target="_blank">
                F-Droid
                <BoxArrowUpRight/>
            </a>
            . Currently, Mavinote is not available from other stores. However, you can build Android and iOS applications from source and use them in your devices.
            For more information, please take a look at the <a rel="external" href="https://github.com/bwqr/mavinote" target="_blank">Mavinote git repository <BoxArrowUpRight/></a>.
        </p>

        <div class="d-flex justify-content-center">
            <a rel="external" href="https://f-droid.org/packages/com.bwqr.mavinote/" target="_blank">
                <img src="https://fdroid.gitlab.io/artwork/badge/get-it-on.png" height="80" alt="get-it-on-f-droid"/>
            </a>
        </div>
    </div>
</section>

<footer class="container-sm py-5 mt-5">
    <div class="row">
        <div class="col col-md-4">
            <h2>Mavinote</h2>
            <p>Open Source Note-Taking Application</p>
        </div>
        <div class="col">
            <ul class="list-unstyled">
                <li><h4>Links</h4></li>
                <li class="my-2"><a class="text-decoration-none text-body" rel="external nofollow" href="https://f-droid.org/packages/com.bwqr.mavinote/">F-Droid</a></li>
                <li class="my-2"><a class="text-decoration-none text-body" rel="external nofollow" href="https://github.com/bwqr/mavinote">GitHub</a></li>
            </ul>
        </div>
    </div>
</footer>

<style lang="scss">
    .octocat {
        fill: var(--bs-light);
        height: 5.2rem;
        position: fixed;
        right: 0;
        top: 0;
        width: 5.2rem;
        z-index: 2;
    }

    .octocat-arm, .octocat-body {
        fill: var(--bs-primary);
    }
</style>
