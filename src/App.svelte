<script>
  import Icon from "svelte-awesome";
  import { calculator } from "svelte-awesome/icons";
  import Num from "./components/Num.svelte";
  import wasm from "../Cargo.toml";

  /// user input number
  let n = 17;

  /// wasm module
  let mod = {
    prime_test: () => null,
    echo: () => null,
  };

  let result = {
    error: false,
    number: null,
    is_prime: null,
  };

  wasm().then(module => {
    console.log("wasm loaded successfully");
    mod.prime_test = module.prime_test;
    mod.echo = module.echo;
  });

  function entry() {
    clear();
    try {
      let num = mod.echo(BigInt(n));
      console.log({n, num});
      result.number = num;
      if (num !== BigInt(n)) { throw `${n} cannot be parsed u64`; }
      result.is_prime = mod.prime_test(num);
      console.log(result);
    } catch(e) {
      result.error = e;
    }
    return false;
  }

  function clear() {
    result.error = false;
    result.number = null;
    result.is_prime = null;
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
    <form on:submit|preventDefault={entry}>
      <div class="field has-addons">
        <div class="control">
          <input class="input"
                 type="text"
                 placeholder="17"
                 bind:value={n}
                 on:keyup|preventDefault={entry}
                 pattern="^[0-9]+$"
                 title="You can input only [0-9]+"
                 required
                 />
        </div>
        <div class="control">
          <a class="button is-info" on:click={entry}>is?</a>
        </div>
      </div>
    </form>
  </div>
</div>

<div class="section">

  <div class="container">
    {#if result.error}
      <div class="notification is-primary is-light">
        <div>
          <b>Something Error!</b>
          <code><pre>
  {result.error}</pre></code>
            <code>{n}</code> is not a <b>Natural</b> Number or <b>TooBig</b> Number?
          Numbers must be unsigned 64bit integers.
        </div>
      </div>
    {/if}
  </div>

  <div class="container">
    {#if !result.error && result.number != null}
      <div>
        <Num value={result.number} /> is a Natural Number.
      </div>
    {/if}
  </div>

  <div class="container">
    {#if !result.error && result.number != null && result.is_prime != null}
      <div>
        <Num value={result.number} /> is {#if result.is_prime}prime{:else}not prime{/if}.
      </div>
    {/if}
  </div>

</div>

<style global lang="scss">
  @import "main.scss";
</style>
