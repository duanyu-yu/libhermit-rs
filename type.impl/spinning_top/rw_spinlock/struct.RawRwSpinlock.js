(function() {
    var type_impls = Object.fromEntries([["hermit_sync",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-RawRwSpinlock%3CR%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#17\">Source</a><a href=\"#impl-Debug-for-RawRwSpinlock%3CR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"spinning_top/rw_spinlock/struct.RawRwSpinlock.html\" title=\"struct spinning_top::rw_spinlock::RawRwSpinlock\">RawRwSpinlock</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"spinning_top/relax/trait.Relax.html\" title=\"trait spinning_top::relax::Relax\">Relax</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#17\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","hermit_sync::rwlock::RawRwSpinLock"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RawRwLock-for-RawRwSpinlock%3CR%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#56\">Source</a><a href=\"#impl-RawRwLock-for-RawRwSpinlock%3CR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; <a class=\"trait\" href=\"lock_api/rwlock/trait.RawRwLock.html\" title=\"trait lock_api::rwlock::RawRwLock\">RawRwLock</a> for <a class=\"struct\" href=\"spinning_top/rw_spinlock/struct.RawRwSpinlock.html\" title=\"struct spinning_top::rw_spinlock::RawRwSpinlock\">RawRwSpinlock</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"spinning_top/relax/trait.Relax.html\" title=\"trait spinning_top::relax::Relax\">Relax</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.INIT\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#58\">Source</a><a href=\"#associatedconstant.INIT\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"lock_api/rwlock/trait.RawRwLock.html#associatedconstant.INIT\" class=\"constant\">INIT</a>: <a class=\"struct\" href=\"spinning_top/rw_spinlock/struct.RawRwSpinlock.html\" title=\"struct spinning_top::rw_spinlock::RawRwSpinlock\">RawRwSpinlock</a>&lt;R&gt;</h4></section></summary><div class='docblock'>Initial value for an unlocked <code>RwLock</code>.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.GuardMarker\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#63\">Source</a><a href=\"#associatedtype.GuardMarker\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"lock_api/rwlock/trait.RawRwLock.html#associatedtype.GuardMarker\" class=\"associatedtype\">GuardMarker</a> = <a class=\"struct\" href=\"lock_api/struct.GuardSend.html\" title=\"struct lock_api::GuardSend\">GuardSend</a></h4></section></summary><div class='docblock'>Marker type which determines whether a lock guard should be <code>Send</code>. Use\none of the <code>GuardSend</code> or <code>GuardNoSend</code> helper types here.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.lock_shared\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#66\">Source</a><a href=\"#method.lock_shared\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.lock_shared\" class=\"fn\">lock_shared</a>(&amp;self)</h4></section></summary><div class='docblock'>Acquires a shared lock, blocking the current thread until it is able to do so.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_lock_shared\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#75\">Source</a><a href=\"#method.try_lock_shared\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.try_lock_shared\" class=\"fn\">try_lock_shared</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Attempts to acquire a shared lock without blocking.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unlock_shared\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#90\">Source</a><a href=\"#method.unlock_shared\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.unlock_shared\" class=\"fn\">unlock_shared</a>(&amp;self)</h4></section></summary><div class='docblock'>Releases a shared lock. <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.unlock_shared\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.lock_exclusive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#97\">Source</a><a href=\"#method.lock_exclusive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.lock_exclusive\" class=\"fn\">lock_exclusive</a>(&amp;self)</h4></section></summary><div class='docblock'>Acquires an exclusive lock, blocking the current thread until it is able to do so.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_lock_exclusive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#106\">Source</a><a href=\"#method.try_lock_exclusive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.try_lock_exclusive\" class=\"fn\">try_lock_exclusive</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Attempts to acquire an exclusive lock without blocking.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unlock_exclusive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#113\">Source</a><a href=\"#method.unlock_exclusive\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.unlock_exclusive\" class=\"fn\">unlock_exclusive</a>(&amp;self)</h4></section></summary><div class='docblock'>Releases an exclusive lock. <a href=\"lock_api/rwlock/trait.RawRwLock.html#tymethod.unlock_exclusive\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_locked\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#120\">Source</a><a href=\"#method.is_locked\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#method.is_locked\" class=\"fn\">is_locked</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Checks if this <code>RwLock</code> is currently locked in any way.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_locked_exclusive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#125\">Source</a><a href=\"#method.is_locked_exclusive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLock.html#method.is_locked_exclusive\" class=\"fn\">is_locked_exclusive</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Check if this <code>RwLock</code> is currently exclusively locked.</div></details></div></details>","RawRwLock","hermit_sync::rwlock::RawRwSpinLock"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RawRwLockDowngrade-for-RawRwSpinlock%3CR%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#142\">Source</a><a href=\"#impl-RawRwLockDowngrade-for-RawRwSpinlock%3CR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; <a class=\"trait\" href=\"lock_api/rwlock/trait.RawRwLockDowngrade.html\" title=\"trait lock_api::rwlock::RawRwLockDowngrade\">RawRwLockDowngrade</a> for <a class=\"struct\" href=\"spinning_top/rw_spinlock/struct.RawRwSpinlock.html\" title=\"struct spinning_top::rw_spinlock::RawRwSpinlock\">RawRwSpinlock</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"spinning_top/relax/trait.Relax.html\" title=\"trait spinning_top::relax::Relax\">Relax</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.downgrade\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#144\">Source</a><a href=\"#method.downgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLockDowngrade.html#tymethod.downgrade\" class=\"fn\">downgrade</a>(&amp;self)</h4></section></summary><div class='docblock'>Atomically downgrades an exclusive lock into a shared lock without\nallowing any thread to take an exclusive lock in the meantime. <a href=\"lock_api/rwlock/trait.RawRwLockDowngrade.html#tymethod.downgrade\">Read more</a></div></details></div></details>","RawRwLockDowngrade","hermit_sync::rwlock::RawRwSpinLock"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RawRwLockRecursive-for-RawRwSpinlock%3CR%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#130\">Source</a><a href=\"#impl-RawRwLockRecursive-for-RawRwSpinlock%3CR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; <a class=\"trait\" href=\"lock_api/rwlock/trait.RawRwLockRecursive.html\" title=\"trait lock_api::rwlock::RawRwLockRecursive\">RawRwLockRecursive</a> for <a class=\"struct\" href=\"spinning_top/rw_spinlock/struct.RawRwSpinlock.html\" title=\"struct spinning_top::rw_spinlock::RawRwSpinlock\">RawRwSpinlock</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"spinning_top/relax/trait.Relax.html\" title=\"trait spinning_top::relax::Relax\">Relax</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.lock_shared_recursive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#132\">Source</a><a href=\"#method.lock_shared_recursive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLockRecursive.html#tymethod.lock_shared_recursive\" class=\"fn\">lock_shared_recursive</a>(&amp;self)</h4></section></summary><div class='docblock'>Acquires a shared lock without deadlocking in case of a recursive lock.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_lock_shared_recursive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#137\">Source</a><a href=\"#method.try_lock_shared_recursive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLockRecursive.html#tymethod.try_lock_shared_recursive\" class=\"fn\">try_lock_shared_recursive</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Attempts to acquire a shared lock without deadlocking in case of a recursive lock.</div></details></div></details>","RawRwLockRecursive","hermit_sync::rwlock::RawRwSpinLock"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RawRwLockUpgrade-for-RawRwSpinlock%3CR%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#154\">Source</a><a href=\"#impl-RawRwLockUpgrade-for-RawRwSpinlock%3CR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; <a class=\"trait\" href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html\" title=\"trait lock_api::rwlock::RawRwLockUpgrade\">RawRwLockUpgrade</a> for <a class=\"struct\" href=\"spinning_top/rw_spinlock/struct.RawRwSpinlock.html\" title=\"struct spinning_top::rw_spinlock::RawRwSpinlock\">RawRwSpinlock</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"spinning_top/relax/trait.Relax.html\" title=\"trait spinning_top::relax::Relax\">Relax</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.lock_upgradable\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#156\">Source</a><a href=\"#method.lock_upgradable\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.lock_upgradable\" class=\"fn\">lock_upgradable</a>(&amp;self)</h4></section></summary><div class='docblock'>Acquires an upgradable lock, blocking the current thread until it is able to do so.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_lock_upgradable\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#165\">Source</a><a href=\"#method.try_lock_upgradable\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.try_lock_upgradable\" class=\"fn\">try_lock_upgradable</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Attempts to acquire an upgradable lock without blocking.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unlock_upgradable\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#180\">Source</a><a href=\"#method.unlock_upgradable\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.unlock_upgradable\" class=\"fn\">unlock_upgradable</a>(&amp;self)</h4></section></summary><div class='docblock'>Releases an upgradable lock. <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.unlock_upgradable\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.upgrade\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#187\">Source</a><a href=\"#method.upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.upgrade\" class=\"fn\">upgrade</a>(&amp;self)</h4></section></summary><div class='docblock'>Upgrades an upgradable lock to an exclusive lock. <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.upgrade\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_upgrade\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#196\">Source</a><a href=\"#method.try_upgrade\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.try_upgrade\" class=\"fn\">try_upgrade</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Attempts to upgrade an upgradable lock to an exclusive lock without\nblocking. <a href=\"lock_api/rwlock/trait.RawRwLockUpgrade.html#tymethod.try_upgrade\">Read more</a></div></details></div></details>","RawRwLockUpgrade","hermit_sync::rwlock::RawRwSpinLock"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RawRwLockUpgradeDowngrade-for-RawRwSpinlock%3CR%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#203\">Source</a><a href=\"#impl-RawRwLockUpgradeDowngrade-for-RawRwSpinlock%3CR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; <a class=\"trait\" href=\"lock_api/rwlock/trait.RawRwLockUpgradeDowngrade.html\" title=\"trait lock_api::rwlock::RawRwLockUpgradeDowngrade\">RawRwLockUpgradeDowngrade</a> for <a class=\"struct\" href=\"spinning_top/rw_spinlock/struct.RawRwSpinlock.html\" title=\"struct spinning_top::rw_spinlock::RawRwSpinlock\">RawRwSpinlock</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"spinning_top/relax/trait.Relax.html\" title=\"trait spinning_top::relax::Relax\">Relax</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.downgrade_upgradable\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#205\">Source</a><a href=\"#method.downgrade_upgradable\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLockUpgradeDowngrade.html#tymethod.downgrade_upgradable\" class=\"fn\">downgrade_upgradable</a>(&amp;self)</h4></section></summary><div class='docblock'>Downgrades an upgradable lock to a shared lock. <a href=\"lock_api/rwlock/trait.RawRwLockUpgradeDowngrade.html#tymethod.downgrade_upgradable\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.downgrade_to_upgradable\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/spinning_top/rw_spinlock.rs.html#214\">Source</a><a href=\"#method.downgrade_to_upgradable\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"lock_api/rwlock/trait.RawRwLockUpgradeDowngrade.html#tymethod.downgrade_to_upgradable\" class=\"fn\">downgrade_to_upgradable</a>(&amp;self)</h4></section></summary><div class='docblock'>Downgrades an exclusive lock to an upgradable lock. <a href=\"lock_api/rwlock/trait.RawRwLockUpgradeDowngrade.html#tymethod.downgrade_to_upgradable\">Read more</a></div></details></div></details>","RawRwLockUpgradeDowngrade","hermit_sync::rwlock::RawRwSpinLock"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[19776]}