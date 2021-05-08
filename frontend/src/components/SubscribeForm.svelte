<script>
    import firebase from "firebase/app";
    import "firebase/messaging";
    import Toastify from 'toastify-js';
    import sendSub from '../api.js';

    var firebaseConfig = {
        apiKey: "AIzaSyAqwahqZV8qMTkixTgfz5Z32HOOnDeKzuI",
        authDomain: "vaxnotify-ba589.firebaseapp.com",
        projectId: "vaxnotify-ba589",
        storageBucket: "vaxnotify-ba589.appspot.com",
        messagingSenderId: "169627591399",
        appId: "1:169627591399:web:7713ba8d11444be36a3842"
    };
    // Initialize Firebase
    firebase.initializeApp(firebaseConfig);

    let pincode = "";
    let age = "";
    let is_loading = false;
    
    async function submitForm() {
        event.preventDefault();
        is_loading = true;
        const messaging = firebase.messaging();
        messaging.getToken({
            vapidKey: 'BHaW3BySv6wyI9NWzbqMUVGamLCrMSz-j-EuCx6AMWOWFsWHKAdH4cYsCrM5kyIQL4-eL-hmsDejMBSu_-ywE-4'
        }).then((current_token) => {
            if (current_token) {
                sendSub(pincode, age, current_token).then(() => {
                    Toastify({
                        text: 'Subscribed Successfully.',
                        duration: 3000,
                        close: true,
                        gravity: 'top',
                        onClick: function(){}
                    }).showToast();
                }).catch(err => {
                    Toastify({
                        text: err,
                        duration: 3000,
                        close: true,
                        gravity: 'top',
                        onClick: function(){}
                    }).showToast();
                }) 
            } else {
                Toastify({
                    text: 'Please allow notifications in the browser to subscribe.',
                    duration: 3000,
                    close: true,
                    gravity: 'top',
                    onClick: function(){}
                }).showToast();
            }
            is_loading = false
        }).catch(err => {
            console.log('An error occurred while retrieving token. ', err)
            Toastify({
                text: 'Please allow notifications in the browser',
                duration: 3000,
                close: true,
                gravity: 'top',
                onClick: function(){}
            }).showToast();
            is_loading = false
        })
    }
</script>

<h1 class="title is-4">Subscribe</h1>
<form on:submit="{submitForm}">
    <div class="field">
    <div class="control">
        <input id="pincode" bind:value="{pincode}" class="input is-medium" type="text" placeholder="Pincode" required>
    </div>
    </div>

    <div class="field">
    <div class="control">
        <input class="input is-medium" bind:value="{age}" type="number" placeholder="Age" required>
    </div>
    </div>
    {#if is_loading}
        <button class="button is-block is-primary is-fullwidth is-medium is-loading">Submit</button>
    {:else}
    <button class="button is-block is-primary is-fullwidth is-medium">Submit</button>
    {/if}

    <br />
    <small><em>Allow Notifications when asked in the browser.</em></small>
</form>