<script>
  import Icon from "svelte-awesome";
  import { calculator } from "svelte-awesome/icons";
  import Footer from "./components/Footer.svelte";
  import wasm from "../Cargo.toml";

  let n = 17;
  let n_inputed = null;
  let prime_test = (n) => null;
  let n_is_prime = null;

  wasm().then(module => {
    console.log("wasm loaded successfully");
    prime_test = module.prime_test;
  });

  function entry() {
    n_inputed = BigInt(n);
    n_is_prime = prime_test(n_inputed);
    console.log({n_is_prime});
  }

  function clear() {
    n_inputed = null;
    n_is_prime = null;
  }
</script>

<section class="hero">
  <div class="hero-body">
    <p class="title">
      <Icon data={calculator} scale="2" />
      cympfh.cc/prime/
    </p>
  </div>
</section>

<div class="section">
  <div class="container">
    <div class="field has-addons">
      <div class="control">
        <input class="input" type="number" placeholder="17" bind:value={n} />
      </div>
      <div class="control">
        <a class="button is-info" on:click={entry}>is?</a>
      </div>
    </div>
  </div>
</div>

<div class="section">
  <div class="container">
    {#if n_inputed != null && n_is_prime != null}
    <div>
      {n_inputed} is {#if n_is_prime}prime{:else}not prime{/if}.
    </div>
    {/if}
  </div>
</div>

<style global lang="scss">
  @import "main.scss";
</style>
