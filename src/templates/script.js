window.addEventListener('load', function () {
    let toggleChatBtn = document.getElementById('toggle-chat-btn');
    let chatHolder = document.getElementById('chat-holder');
    let chatItems = document.getElementById('chat-items');
    let chatDetail = document.getElementById('chat-detail');
    let chatBackBtn = document.getElementById('chat-back');

    toggleChatBtn.addEventListener('click', function () {
        state = toggleChatBtn.getAttribute('data-state');
        if (state === 'closed') {
            toggleChatBtn.setAttribute('data-state', 'open');
            toggleChatBtn.innerHTML = 'close chat';
            chatHolder.style.display = 'block';
            chatHolder.classList.remove('closed-chat');
        } else {
            toggleChatBtn.setAttribute('data-state', 'closed');
            toggleChatBtn.innerHTML = 'open chat';
            chatHolder.style.display = 'none';
            chatHolder.classList.add('closed-chat');
        }
    });

    chatBackBtn.addEventListener('click', function () {
        chatDetail.style.display = 'none';
        chatItems.style.display = 'block';
    });

    document.addEventListener('click', function (e) {
        let classes = [];
        function getClasses(element) {
            if (element.classList) {
                classes.push(element.className);
            }
            if (element.parentElement) {
                getClasses(element.parentElement);
            }
        }
        getClasses(e.target);

        if (classes.includes('chat-item')) {
            chatItems.style.display = 'none';
            chatDetail.style.display = 'block';
        }
    });

});