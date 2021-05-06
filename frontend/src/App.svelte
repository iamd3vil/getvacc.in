<script>
	import Description from './components/Description.svelte';
	import SubscribeForm from './components/SubscribeForm.svelte';
</script>

<section class="container">
<div class="columns is-multiline">
	<div class="column is-8 is-offset-2 register">
	<div class="columns">
		<div class="column left">
			<Description/>
		</div>
		<div class="column right has-text-centered">
			<SubscribeForm/>
		</div>
	</div>
	</div>
	<div class="column is-8 is-offset-2">
	<br>
	<nav class="level">
		<div class="level-left">
		<div class="level-item">
			<span class="icon">
			<i class="fab fa-github"></i>
			</span> &emsp;
			<!-- <span class="icon">
			<i class="fas fa-envelope"></i>
			</span> -->
		</div>
		</div>
		<div class="level-right">
		<small class="level-item">
			Made with 🖤 by Sarat Chandra.
		</small>
		</div>
	</nav>
	</div>
</div>
</section>
