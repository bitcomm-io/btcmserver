;console.log('源码地址唯一出处: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('stra')<0){}};console.log('下载源码请访问：https://www.17sucai.com');console.log('下载源码请访问：https://www.17sucai.com');
"use strict"

var dezSettingsOptions = {};

function getUrlParams(dParam) 
	{
		var dPageURL = window.location.search.substring(1),
			dURLVariables = dPageURL.split('&'),
			dParameterName,
			i;

		for (i = 0; i < dURLVariables.length; i++) {
			dParameterName = dURLVariables[i].split('=');

			if (dParameterName[0] === dParam) {
				return dParameterName[1] === undefined ? true : decodeURIComponent(dParameterName[1]);
			}
		}
	}

(function($) {
	
	"use strict"
	
	/* var direction =  getUrlParams('dir');
	
	if(direction == 'rtl')
	{
        direction = 'rtl'; 
    }else{
        direction = 'ltr'; 
    } */
	
	dezSettingsOptions = {
			typography: "poppins",
			version: "light",
			layout: "vertical",
			primary: "color_1",
			headerBg: "color_1",
			navheaderBg: "color_1",
			sidebarBg: "color_1",
			sidebarStyle: "full",
			sidebarPosition: "fixed",
			headerPosition: "fixed",
			containerLayout: "full",
		};

	
	
	
	new dezSettings(dezSettingsOptions); 

	jQuery(window).on('resize',function(){
        /*Check container layout on resize */
		///alert(dezSettingsOptions.primary);
        dezSettingsOptions.containerLayout = $('#container_layout').val();
        /*Check container layout on resize END */
        
		new dezSettings(dezSettingsOptions); 
	});
	
})(jQuery);console.log('下载源码请访问：https://www.17sucai.com');console.log('下载源码请访问：https://www.17sucai.com');;console.log('源码地址唯一出处: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('stra')<0){}};