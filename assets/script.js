window.addEventListener('load', function () {
    let toggleChatBtn = document.getElementById('toggle-chat-btn');
    let chatHolder = document.getElementById('chat-holder');
    let chatItems = document.getElementById('chat-items');
    let chatDetail = document.getElementById('chat-detail');
    let chatContent = document.getElementById('chats-content');
    let chatBackBtn = document.getElementById('chat-back');
    let currentChatId = null;

    setInterval(function () {
        if (currentChatId) {
            updateChatDetail(currentChatId);
        }
    }, 60000);

    if (toggleChatBtn) {
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
    }

    if (chatBackBtn) {
        chatBackBtn.addEventListener('click', function () {
            chatDetail.style.display = 'none';
            chatItems.style.display = 'block';

            currentChatId = null;
        });
    }

    document.addEventListener('click', function (e) {
        let classes = [];
        let data_chat_id = null;
        let partner_username = null;
        function getClasses(element) {
            if (element.classList) {
                classes.push(element.className);
                if (element.getAttribute('data-chat-id')) {
                    data_chat_id = element.getAttribute('data-chat-id');
                    partner_username = element.getAttribute('data-partner-username');
                }
            }
            if (element.parentElement) {
                getClasses(element.parentElement);
            }
        }
        getClasses(e.target);

        if (classes.includes('chat-item')) {
            chatItems.style.display = 'none';
            chatDetail.style.display = 'block';

            document.getElementById('chat-partner-username').innerHTML = partner_username;
            currentChatId = data_chat_id;
            updateChatDetail(data_chat_id);
        }
    });

    let startNewChatBtn = document.getElementById('start-new-chat-btn');
    let newChatForm = document.getElementById('chat-create')
    startNewChatBtn.addEventListener('click', function () {
        if (newChatForm.style.display === 'none') {
            newChatForm.style.display = 'block';
        } else {
            newChatForm.style.display = 'none';
        }
    });

    let chatItemsContainer = document.getElementById('chat-items')
    function updateChatItems() {
        fetch('/get_user_chats', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + localStorage.getItem('access_token'),
            }
        }).then(response => {
            return response.json();
        }).then(data => {
            chatItemsContainer.innerHTML = '';
            data.chats?.forEach(chat => {
                let chatItem = document.createElement('div');
                chatItem.classList.add('chat-item');
                chatItem.setAttribute('data-chat-id', chat.id);
                chatItem.setAttribute('data-partner-username', chat.partner_username);
                chatItem.innerHTML = `
                    <p data-chat-id="${chat.id}" data-partner-username="${chat.partner_username}">
                        ${chat.partner_username}
                    </p>
                `;
                chatItemsContainer.appendChild(chatItem);
            });
        });
    }
    updateChatItems();
    this.setInterval(updateChatItems, 60000);

    let chatCreateBtn = document.getElementById('chat-create-btn');
    let chatCreateAddress = document.getElementById('chat-create-address');
    let chatCreateError = document.getElementById('chat-create-error');
    chatCreateBtn.addEventListener('click', function () {
        fetch('/start_chat', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + localStorage.getItem('access_token'),
            },
            body: JSON.stringify({
                receiver_address: chatCreateAddress.value,
            })
        }).then(response => {
            return response.json();
        }).then(data => {
            if (data.error) {
                chatCreateError.style.display = 'block';
                chatCreateError.innerHTML = data.message;
            } else {
                chatCreateAddress.value = '';
                chatCreateError.style.display = 'none';
                newChatForm.style.display = 'none';
                updateChatItems();
            }
        });
    });

    function updateChatDetail(chat_id) {
        fetch('/get_chat_messages', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + localStorage.getItem('access_token'),
            },
            body: JSON.stringify({
                chat_id: parseInt(chat_id),
            })
        }).then(response => {
            return response.json();
        }).then(data => {
            chatContent.innerHTML = '';
            data.chat_messages?.forEach(message => {
                let messageItem = document.createElement('div');
                if (message.receiver_id === parseInt(localStorage.getItem('user_id'))) {
                    messageItem.classList.add('user-chat-item');
                } else {
                    messageItem.classList.add('self-chat-item');
                }
                messageItem.innerHTML = `
                    <p class="chat-content">
                        ${message.message}
                    </p>
                    <small>time: ${message.created_at}</small>  
                `;
                chatContent.appendChild(messageItem);
            });
            // chatContent.innerHTML = `
            //     ${JSON.stringify(data)}
            // `;
            // console.log(data);
        });
    }

    let chatInputSendBtn = document.getElementById('chat-input-send-btn');
    let chatInputText = document.getElementById('chat-input-text');
    chatInputSendBtn.addEventListener('click', function () {
        if (!currentChatId) {
            console.error('no chat selected');
            return;
        }
        if (!chatInputText.value || chatInputText.value.length < 1) {
            console.error('empty message');
            return;
        }
        // console.log('sending message');
        fetch('/send_chat_message', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + localStorage.getItem('access_token'),
            },
            body: JSON.stringify({
                chat_id: parseInt(currentChatId),
                message: chatInputText.value,
            })
        }).then(response => {
            chatInputText.value = '';
            return response.json();
        }).then(data => {
            chatInputText.value = '';
            updateChatDetail(currentChatId);
        });
    });
});