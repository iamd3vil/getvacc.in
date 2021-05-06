<script>
    import firebase from "firebase/app";
    import "firebase/messaging";
    import axios from 'axios';
    import Toastify from 'toastify-js';

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
                    is_loading = false
                    console.log("sent api request")
                    Toastify({
                        text: 'Subscribed Successfully.',
                        duration: 3000,
                        close: true,
                        gravity: 'top',
                        style: {
                            background: 'green'
                        },
                        onClick: function(){}
                    }).showToast();
                })
            } else {
                console.log('No registration token available. Request permission to generate one.');
            }
        }).catch(err => {
            console.log('An error occurred while retrieving token. ', err)
        })
        console.log(pincode, age);
    }

    async function sendSub(pincode, age, token) {
        try {
            const response = await axios.post('/api/v1/subscribe', {
                pincode: pincode,
                age_limit: age,
                token: token,
            })

            console.log('response: ', response)
        } catch (error) {
            console.log('Error sending API call: ', error)
        }
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