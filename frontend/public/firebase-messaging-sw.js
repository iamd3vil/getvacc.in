importScripts('https://www.gstatic.com/firebasejs/8.5.0/firebase-app.js');
importScripts('https://www.gstatic.com/firebasejs/8.5.0/firebase-messaging.js');

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

const messaging = firebase.messaging();

messaging.onBackgroundMessage(payload => {
    console.log('[firebase-messaging-sw.js] Received background message ', payload);
});