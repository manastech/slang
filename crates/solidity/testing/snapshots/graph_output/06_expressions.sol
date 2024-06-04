contract Foo {
    address owner;
    uint32 score;

    function bar() {
        if (msg.sender == owner) {
            score = score + 1;
        }
    }
}
