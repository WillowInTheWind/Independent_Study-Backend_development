import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LogincallbackComponent } from './logincallback.component';

describe('LogincallbackComponent', () => {
  let component: LogincallbackComponent;
  let fixture: ComponentFixture<LogincallbackComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LogincallbackComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(LogincallbackComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
