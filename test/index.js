/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/hUdemy.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly
  )
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

/*

orchestrator.registerScenario("Scenario1: Create new course", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course test 1"
    }
  );
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();

  const courseResult = await bob.call("course_dna", "courses", "get_entry", {
    address: course_addr.Ok
  });
  const course = JSON.parse(courseResult.Ok.App[1]);
  console.log(course);
  t.deepEqual(course, {
    title: "course test 1",
    teacher_address: alice.instance("course_dna").agentAddress,
    modules: []
  });
  // Wait for all network activity to settle
  await s.consistency();
});

orchestrator.registerScenario("Scenario2: Update course title", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "new course test for update test"
    }
  );

  const course_update_addrss = await alice.call(
    "course_dna",
    "courses",
    "update_course",
    {
      title: "course title updated",
      course_address: course_addr.Ok,
      modules_addresses: []

    }
  );



  const courseResult = await bob.call("course_dna", "courses", "get_entry", {
    address: course_update_addrss.Ok
  });

  const course = JSON.parse(courseResult.Ok.App[1]);
  console.log(course);
  t.deepEqual(course, {
    title: "course title updated",
    teacher_address: alice.instance("course_dna").agentAddress,
    modules: []
  });



});

orchestrator.registerScenario("Scenario3: Delete course", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "new course test for delete scenario"
    }
  );
  await s.consistency();

  const delete_result = await alice.call(
    "course_dna",
    "courses",
    "delete_course",
    {
      course_address: course_addr.Ok,
    }
  );
  await s.consistency();

  t.ok(delete_result.Ok);

  const courseResult = await bob.call("course_dna", "courses", "get_entry", {
    address: delete_result.Ok
  });
  t.deepEqual(courseResult.Ok.Deletion.deleted_entry_address, course_addr.Ok);
  // t.deepEqual(courseResult, {
  //   Ok:
  //   {
  //     Deletion: {
  //       deleted_entry_address: delete_result.Ok
  //     }
  //   }
  // });

  await s.consistency();

});

*/
orchestrator.registerScenario("Scenario4: Create new Module for a Course", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 4"
    }
  );
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();

  const new_module_addr = await bob.call("course_dna", "courses", "create_module", {
    title: "module 1 for course 1",
    course_address: course_addr.Ok
  });

  console.log("hedayat abedijoo");
  console.log(new_module_addr);
  t.notOk(new_module_addr.Ok);
  // await s.consistency();

  // const moduleResult = await bob.call("course_dna", "courses", "get_entry", {
  //   address: new_module_addr.Ok
  // });
  // const module = JSON.parse(moduleResult.Ok.App[1]);
  // console.log(module);
  // t.deepEqual(module, {
  //   title: "module 1 for course 1",
  //   course_address: course_addr.Ok
  // });

  // const courseResult = await bob.call("course_dna", "courses", "get_entry", {
  //   address: course_addr.Ok
  // });
  // const course = JSON.parse(courseResult.Ok.App[1]);
  // console.log(course);
  // t.deepEqual(course, {
  //   title: "course for scenario 4",
  //   teacher_address: alice.instance("course_dna").agentAddress,
  //   modules: [new_module_addr]
  // });
  // Wait for all network activity to settle
  await s.consistency();
});

// orchestrator.registerScenario("description of example test", async (s, t) => {
//   const { alice, bob } = await s.players(
//     { alice: conductorConfig, bob: conductorConfig },
//     true
//   );

//   const result = await alice.call("course_dna", "courses", "hello_holo", {
//     title: "every thing is working"
//   });
//   t.deepEqual(result, { Ok: "every thing is working" });
//   await s.consistency();

//   console.log("Test1 create_course:");
//   const course_addr = await alice.call(
//     "course_dna",
//     "courses",
//     "create_course",
//     {
//       title: "first test course"
//     }
//   );
//   console.log(course_addr);
//   t.ok(course_addr.Ok);
//   // Wait for all network activity to settle
//   await s.consistency();

//   const courseResult = await bob.call("course_dna", "courses", "get_entry", {
//     address: course_addr.Ok
//   });
//   const course = JSON.parse(courseResult.Ok.App[1]);
//   console.log("Test2 Get Course:");
//   console.log(course);
//   t.deepEqual(course, {
//     title: "first test course",
//     teacher_address: alice.instance("course_dna").agentAddress,
//     modules: []
//   });
//   // Wait for all network activity to settle
//   await s.consistency();

//   let moduleAddr = await bob.call("course_dna", "courses", "create_module", {
//     title: "new module",
//     course_address: course_addr.Ok
//   });
//   t.notOk(moduleAddr.Ok);

//   moduleAddr = await alice.call("course_dna", "courses", "create_module", {
//     title: "new module",
//     course_address: course_addr.Ok
//   });
//   t.ok(moduleAddr.Ok);
//   await s.consistency();

//   console.log("Test3: Create Module:");
//   console.log(moduleAddr.Ok);
//   const moduleResult = await bob.call("course_dna", "courses", "get_entry", {
//     address: moduleAddr.Ok
//   });

//   const module = JSON.parse(moduleResult.Ok.App[1]);

//   const updatedCourseResult = await alice.call(
//     "course_dna",
//     "courses",
//     "get_entry",
//     {
//       address: module.course_address
//     }
//   );
//   const updatedCourse = JSON.parse(updatedCourseResult.Ok.App[1]);

//   console.log("Test4: Get Course Again:");
//   console.log(updatedCourseResult.Ok);

//   // const something = JSON.parse(module_result.Ok);
//   // console.log(something);

//   t.deepEqual(updatedCourse, {
//     title: "first test course",
//     teacher_address: alice.instance("course_dna").agentAddress,
//     modules: [moduleAddr.Ok]
//   });

//   //const course = await bob.call("course_dna", "courses", "get_course", { "address": module_result.Ok });

//   // Wait for all network activity to settle
//   await s.consistency();
//   // // check for equality of the actual and expected results
//   // // t.deepEqual(result, { Ok: { App: ['my_entry', '{"content":"sample content"}'] } })
//   // t.ok(result.Ok);
// });

orchestrator.run();
