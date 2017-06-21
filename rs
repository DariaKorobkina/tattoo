body {
	margin: 0;
}
#wrapper {
	display: flex;
	min-height: 100vh;
	flex-direction: column;
}

#header {
	padding: 22px 32px 22px 32px;
	position: fixed;
	display: flex;
	z-index: 11;
	top: 0;
	left: 0;
	right: 0;
	justify-content: space-between;
	background-color: #F5F5F5;
}

#header .holder {
	display: flex;
	justify-content: flex-end;
	align-items: center;
}

.mob-holder {
	display: flex;
}

.logo {
	width: 160px;
}
    
.navigation ul {
	display: flex;
	justify-content: flex-end;
	align-items: center;
	list-style: none;
	margin: 0px;
	padding: 0px;
	font-size: 20px;
    line-height: 22px;
    letter-spacing: 1px
}

.navigation li {
	margin: 0 0 0 41px;
}
.navigation a {
	text-decoration: none;
	color: #000;
}

#main {
	flex: 1 0 auto;
	padding: 98px 0 0 0;
	display: block;
}
.home-banner {
	background-image: url(banner.jpg);
}
.home-banner {
	display: flex;
	height: 750px;
	flex-direction: column;
	justify-content: center;
	position: relative;
	background-size: cover;
	margin:0;
	overflow: hidden;
}

.home-banner >.container {
	width: 100%;
	max-width: 1031px;
}

.container {
	padding: 0 15px 0 15px;
	margin: 0 auto 0 auto;
}

.banner-holder {
	justify-content: space-between;
	padding: 25px;
}

.banner-holder .banner-img {
	width: 100%;
}
.banner-img img {
	width: 38%;
	padding: 10px;
}
.banner-text p{
	width: 100%;
	color: #fff;
	font-size: 20px;
	line-height: 24px;
	letter-spacing: 1px;
	padding: 10px;
}
.btn-form {
	padding: 10px;
}

button {
	width: 300px;
    height: 50px;
    background-color: red;
    border: none;
    font-size: 14px;
    color: #fff;
    letter-spacing: 1px;
}

.founders .info{
	display: flex;
	justify-content: space-between;
}
.info{
	display: block;
	max-width: 1031px;
	margin: 0 auto 0 auto;
}
.founders h2{
	font-size: 36px;
	padding: 50px 10px 10px 32px;
}

.info .person {
	display: flex;
	width: 30%;
	flex-direction: column;
	align-items: center;
}

.person img {
	width: 100%;
}
.person h3 {
	font-size: 24px;
}
