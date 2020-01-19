/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require('path')

const { Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/hUdemy.dna.json")

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape')),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,


  ),
});

const dna = Config.dna(dnaPath, "course_dna");
const conductorConfig = Config.gen(
  { course_dna: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    }
  }
);

// const orchestrator = new Orchestrator({
//   waiter: {
//     softTimeout: 20000,
//     hardTimeout: 30000
//   }
// });

orchestrator.registerScenario("description of example test",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: conductorConfig, bob: conductorConfig },
      true
    );


    const result = await alice.call('course_dna', 'courses', 'hello_holo', { title: "every thing is working" });
    t.deepEqual(result, { Ok: 'every thing is working' });
    await s.consistency();

    console.log("Test create_course:");

    // Make a call to a Zome function
    // indicating the function, and passing it an input
    const addr = await alice.call("course_dna", "courses", "create_course", {
      title: "first test course"
    });
    console.log(addr);
    t.ok(addr.Ok);
    await s.consistency()

    console.log("Test get_course:");

    const result2 = await bob.call("course_dna", "courses", "get_course", { "address": addr.Ok })
    console.log(result2);

    // Wait for all network activity to settle
    await s.consistency()
    // // check for equality of the actual and expected results
    // // t.deepEqual(result, { Ok: { App: ['my_entry', '{"content":"sample content"}'] } })
    // t.ok(result.Ok);

  })

orchestrator.run()
